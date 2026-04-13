package com.dcz.mrecord.service.impl;

import cn.hutool.core.lang.Validator;
import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import cn.hutool.crypto.digest.BCrypt;
import com.dcz.mrecord.bo.MailParamsBO;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.dto.UserRegisterDTO;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.SysUserMapper;
import com.dcz.mrecord.service.EmailService;
import com.dcz.mrecord.service.SysConfigService;
import com.dcz.mrecord.service.SysUserService;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import org.springframework.stereotype.Service;

/**
 * 用户服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysUserServiceImpl extends ServiceImpl<SysUserMapper, SysUser> implements SysUserService {
    @Resource
    private SysUserMapper userMapper;

    @Resource
    private EmailService emailService;

    @Resource
    private SysConfigService sysConfigService;

    /**
     * 用户注册
     *
     * @param params 注册参数
     * @return 注册结果
     */
    @Override
    public String userRegister(UserRegisterDTO params) {
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
     * 获取注册成功邮件参数
     *
     * @return 邮件参数
     */
    private MailParamsBO getRegisterSuccessEmailParam(SysUser user) {
        MailParamsBO mailParamsBO = new MailParamsBO();
        mailParamsBO.setTo(user.getEmail());
        mailParamsBO.setUserName(user.getNickname());
        mailParamsBO.setUserEmail(user.getEmail());
        mailParamsBO.setWebSite(sysConfigService.getWebSite());
        return mailParamsBO;
    }
}
