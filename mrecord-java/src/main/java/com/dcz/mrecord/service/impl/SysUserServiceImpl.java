package com.dcz.mrecord.service.impl;

import cn.hutool.core.lang.Validator;
import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import cn.hutool.crypto.SecureUtil;
import cn.hutool.crypto.digest.BCrypt;
import cn.hutool.crypto.symmetric.AES;
import com.dcz.mrecord.bo.MailParamsBO;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.config.MrConf;
import com.dcz.mrecord.dto.UserDTO;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.SysUserMapper;
import com.dcz.mrecord.service.EmailService;
import com.dcz.mrecord.service.SysConfigService;
import com.dcz.mrecord.service.SysUserService;
import com.dcz.mrecord.util.JwtUtil;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;

/**
 * 用户服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@Service
public class SysUserServiceImpl extends ServiceImpl<SysUserMapper, SysUser> implements SysUserService {
    @Resource
    private SysUserMapper userMapper;

    @Resource
    private EmailService emailService;
    @Resource
    private SysConfigService sysConfigService;

    @Resource
    private JwtUtil jwtUtil;
    @Resource
    private MrConf mrConf;

    /**
     * 用户注册
     *
     * @param params 注册参数
     * @return 注册结果
     */
    @Override
    public String userRegister(UserDTO params) {
        // 邮箱验证
        String email = params.getEmail();
        if (StrUtil.isBlankIfStr(email)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "邮箱不能为空");
        }
        if (!Validator.isEmail(email)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "邮箱格式错误");
        }

        // 密码验证
        String password = params.getPassword();
        if (StrUtil.isBlankIfStr(password)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "密码不能为空");
        }
        if (password.length() < 6) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "密码长度不能小于6");
        }

        // 昵称验证
        String nickname = params.getNickname();
        if (StrUtil.isBlankIfStr(nickname)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "昵称不能为空");
        }

        // 邮箱是否已注册
        if (userMapper.selectOneByQuery(QueryWrapper.create().and(SysUser::getEmail).eq(email)) != null) {
            throw new MrecordException(ResCode.UNAUTHORIZED);
        }

        SysUser user = new SysUser();
        user.setId(IdUtil.simpleUUID());
        user.setEmail(email);
        user.setNickname(nickname);

        // 加密密码
        String hashedPassword = BCrypt.hashpw(password, BCrypt.gensalt());
        user.setPassword(hashedPassword);

        int insert = userMapper.insert(user);
        if (insert <= 0) {
            throw new MrecordException(ResCode.DATA_NOT_EXIST.getCode(), "用户注册失败，请联系管理员");
        }

        // 发送注册成功邮件
        emailService.sendRegisterSuccessEmail(getRegisterSuccessEmailParam(user));
        return user.getEmail();
    }

    /**
     * 用户登录
     *
     * @param params 登录参数
     * @return 登录结果
     */
    @Override
    public String login(UserDTO params) {
        String email = params.getEmail();

        // 邮箱验证
        SysUser sysUser = userMapper.selectOneByQuery(QueryWrapper.create().and(SysUser::getEmail).eq(email));
        if (sysUser == null) {
            throw new MrecordException(ResCode.LOGIN_INFO_ERROR);
        }

        // 密码验证
        String password = params.getPassword();
        if (!BCrypt.checkpw(password, sysUser.getPassword())) {
            throw new MrecordException(ResCode.LOGIN_INFO_ERROR);
        }

        // 状态验证
        if (sysUser.getStatus() != 0) {
            throw new MrecordException(ResCode.USER_STATUS_ERROR);
        }

        return jwtUtil.createToken(sysUser.getId());
    }

    /**
     * 忘记密码
     *
     * @param params 忘记密码参数
     */
    @Override
    public void forgotPassword(UserDTO params) throws Exception {
        String email = params.getEmail();

        // 邮箱验证
        SysUser sysUser = userMapper.selectOneByQuery(QueryWrapper.create().and(SysUser::getEmail).eq(email));
        if (sysUser == null) {
            log.warn("未查找到该用户，尝试找回密码。{}", params);
            return;
        }
        // 状态验证
        if (sysUser.getStatus() != 0) {
            log.warn("用户状态异常，尝试找回密码。{}", sysUser);
            return;
        }
        // 发送找回密码邮件
        emailService.sendRetrievePasswordEmail(getForgotPasswordEmailParam(sysUser, getResetPasswordUrl(sysUser)));
    }

    /**
     * 重置密码
     *
     * @param params 重置密码参数
     */
    @Override
    public void resetPassword(UserDTO params) {
        String token = params.getRePasswordToken();
        if (StrUtil.isBlankIfStr(token)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "重置密码令牌不能为空");
        }

        SysUser sysUser = checkRePasswordToken(token);
        if (sysUser == null) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "重置密码令牌错误");
        }

        String password = params.getPassword();
        if (StrUtil.isBlankIfStr(password)) {
            throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "密码不能为空");
        }

        String hashedPassword = BCrypt.hashpw(password, BCrypt.gensalt());
        sysUser.setPassword(hashedPassword);
        userMapper.updateByQuery(sysUser, QueryWrapper.create().and(SysUser::getId).eq(sysUser.getId()));
    }

    /**
     * 获取重置密码链接
     *
     * @param user 用户
     * @return 重置密码链接
     */
    private String getResetPasswordUrl(SysUser user) {
        String userId = user.getId();

        // 过期时间：15分钟后
        long expireTime = System.currentTimeMillis() + 15 * 60 * 1000;
        // 防伪造随机串
        String randomStr = IdUtil.fastSimpleUUID();
        String plainText = userId + "_" + expireTime + "_" + randomStr;

        // AES加密
        String token = SecureUtil.aes(mrConf.getResetPwdTokenSecret().getBytes()).encryptBase64(plainText);

        return sysConfigService.getWebSite() + "reset-password?token=" + URLEncoder.encode(token, StandardCharsets.UTF_8);
    }

    /**
     * 检查重置密码令牌
     *
     * @param token 重置密码令牌
     * @return 用户对象
     */
    private SysUser checkRePasswordToken(String token) {
        try {
            // 解密
            AES aes = SecureUtil.aes(mrConf.getResetPwdTokenSecret().getBytes());
            String plainText = aes.decryptStr(token);

            // 拆分三部分：主键、过期时间、防伪造随机串
            String[] arr = plainText.split("_");
            Long userId = Long.valueOf(arr[0]);
            long expireTime = Long.parseLong(arr[1]);
            // String randomStr = arr[2];

            // 判断是否过期
            if (System.currentTimeMillis() > expireTime) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "重置密码令牌已过期");
            }

            // 根据 userId 去查用户，允许重置密码
            return userMapper.selectOneById(userId);
        } catch (Exception e) {
            log.error("重置密码令牌解析失败", e);
            return null;
        }
    }

    /**
     * 获取注册成功邮件参数
     *
     * @return 邮件参数
     */
    private MailParamsBO getRegisterSuccessEmailParam(SysUser user) {
        MailParamsBO mailParamsBO = new MailParamsBO();
        mailParamsBO.setTo(user.getEmail());
        mailParamsBO.setUserName(user.getNickname());
        mailParamsBO.setUserEmail(user.getEmail());
        return mailParamsBO;
    }

    /**
     * 获取忘记密码邮件参数
     *
     * @param user    用户
     * @param repwUrl 重置密码链接
     * @return 邮件参数
     */
    private MailParamsBO getForgotPasswordEmailParam(SysUser user, String repwUrl) {
        MailParamsBO mailParamsBO = new MailParamsBO();
        mailParamsBO.setTo(user.getEmail());
        mailParamsBO.setUserName(user.getNickname());
        mailParamsBO.setUserEmail(user.getEmail());
        mailParamsBO.setRepassword(repwUrl);
        return mailParamsBO;
    }
}
