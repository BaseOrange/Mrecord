package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.bo.EmailConfigBo;
import com.dcz.mrecord.entity.SysConfig;
import com.dcz.mrecord.mapper.SysConfigMapper;
import com.dcz.mrecord.service.SysConfigService;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

/**
 * 配置项服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@Service
public class SysConfigServiceImpl extends ServiceImpl<SysConfigMapper, SysConfig> implements SysConfigService {

    /**
     * 缓存邮件配置
     */
    private static EmailConfigBo EMAIL_CONFIG = null;
    @Resource
    private SysConfigMapper sysConfigMapper;

    /**
     * 刷新缓存
     */
    @Override
    public void refreshCache() {
        EMAIL_CONFIG = null;
    }

    /**
     * 获取邮件配置
     *
     * @return 邮件配置
     */
    @Override
    public EmailConfigBo getEmailConfig() {
        if (EMAIL_CONFIG != null) {
            return EMAIL_CONFIG;
        }

        QueryWrapper queryWrapper = new QueryWrapper();
        queryWrapper.likeLeft(SysConfig::getKey, "mail");
        List<SysConfig> sysConfigs = sysConfigMapper.selectListByQuery(queryWrapper);
        if (sysConfigs == null || sysConfigs.isEmpty()) {
            log.warn("管理员未配置邮件参数，返回空邮箱配置信息。");
            return null;
        }

        // 构建配置文件
        EmailConfigBo emailConfigBo = new EmailConfigBo();
        // 获取邮件服务器
        Optional<SysConfig> hostAny = sysConfigs.stream().filter(s -> "mail.hostName".equals(s.getKey())).findAny();
        if (hostAny.isPresent()) {
            emailConfigBo.setHostName(hostAny.get().getValue());
        } else {
            log.warn("管理员未配置邮件服务器，返回空邮箱配置信息。");
            return null;
        }

        // 获取邮件端口
        Optional<SysConfig> portAny = sysConfigs.stream().filter(s -> "mail.smtpPort".equals(s.getKey())).findAny();
        if (portAny.isPresent()) {
            emailConfigBo.setSmtpPort(Integer.parseInt(portAny.get().getValue()));
        } else {
            log.warn("管理员未配置邮件端口，返回空邮箱配置信息。");
            return null;
        }

        // 获取邮件SSL
        Optional<SysConfig> sslAny = sysConfigs.stream().filter(s -> "mail.ssl".equals(s.getKey())).findAny();
        sslAny.ifPresent(sysConfig -> emailConfigBo.setSsl(Boolean.parseBoolean(sysConfig.getValue())));

        // 获取邮件用户名
        Optional<SysConfig> usernameAny = sysConfigs.stream().filter(s -> "mail.userName".equals(s.getKey())).findAny();
        if (usernameAny.isPresent()) {
            emailConfigBo.setUsername(usernameAny.get().getValue());
        } else {
            log.warn("管理员未配置邮件用户名，返回空邮箱配置信息。");
            return null;
        }

        // 获取邮件密码
        Optional<SysConfig> passwordAny = sysConfigs.stream().filter(s -> "mail.password".equals(s.getKey())).findAny();
        if (passwordAny.isPresent()) {
            emailConfigBo.setPassword(passwordAny.get().getValue());
        } else {
            log.warn("管理员未配置邮件密码，返回空邮箱配置信息。");
            return null;
        }

        // 获取邮件发件人
        Optional<SysConfig> fromAny = sysConfigs.stream().filter(s -> "mail.from".equals(s.getKey())).findAny();
        if (fromAny.isPresent()) {
            emailConfigBo.setFrom(fromAny.get().getValue());
        } else {
            log.warn("管理员未配置发件人邮箱，返回空邮箱配置信息。");
            return null;
        }

        EMAIL_CONFIG = emailConfigBo;
        return emailConfigBo;
    }
}
