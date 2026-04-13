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

    /**
     * 网站站点
     */
    private static String WEB_SITE = null;

    /**
     * 管理员邮箱
     */
    private static String ADMIN_MAIL = null;

    @Resource
    private SysConfigMapper sysConfigMapper;

    /**
     * 刷新缓存
     */
    @Override
    public void refreshCache() {
        EMAIL_CONFIG = null;
        WEB_SITE = null;
        ADMIN_MAIL = null;
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

    /**
     * 获取网站站点
     *
     * @return 网站站点
     */
    @Override
    public String getWebSite() {
        if (WEB_SITE != null){
            return WEB_SITE;
        }

        QueryWrapper queryWrapper = new QueryWrapper();
        queryWrapper.eq(SysConfig::getKey, "webSite");
        List<SysConfig> sysConfigs = sysConfigMapper.selectListByQuery(queryWrapper);

        if (sysConfigs == null || sysConfigs.isEmpty()) {
            log.warn("管理员未配置站点地址参数，返回空站点配置信息。");
            return null;
        }

        WEB_SITE = sysConfigs.get(0).getValue();
        return WEB_SITE;
    }

    /**
     * 获取管理员邮箱
     *
     * @return 管理员邮箱
     */
    @Override
    public String getAdminMail() {
        if (ADMIN_MAIL != null) {
            return ADMIN_MAIL;
        }

        QueryWrapper queryWrapper = new QueryWrapper();
        queryWrapper.eq(SysConfig::getKey, "adminMail");
        List<SysConfig> sysConfigs = sysConfigMapper.selectListByQuery(queryWrapper);

        if (sysConfigs == null || sysConfigs.isEmpty()) {
            log.warn("管理员未配置管理员邮箱参数，返回空管理员邮箱配置信息。");
            return null;
        }

        ADMIN_MAIL = sysConfigs.get(0).getValue();
        return ADMIN_MAIL;
    }
}
