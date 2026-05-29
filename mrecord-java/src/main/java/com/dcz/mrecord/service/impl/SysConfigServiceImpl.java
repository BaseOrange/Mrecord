package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.bo.EmailConfigBo;
import com.dcz.mrecord.bo.SiteConfigBo;
import com.dcz.mrecord.dto.UpdateEmailConfigDTO;
import com.dcz.mrecord.dto.UpdateSiteConfigDTO;
import com.dcz.mrecord.entity.SysConfig;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.mapper.SysConfigMapper;
import com.dcz.mrecord.mapper.SysUserMapper;
import com.dcz.mrecord.service.SysConfigService;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;

/**
 * 配置项服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@Service
public class SysConfigServiceImpl extends ServiceImpl<SysConfigMapper, SysConfig> implements SysConfigService {

    private static final String MASKED_PASSWORD = "******";

    private final Object cacheLock = new Object();

    /**
     * 缓存邮件配置(volatile 保证可见性,双重检查锁保证发布安全)
     */
    private volatile EmailConfigBo emailConfigCache;

    /**
     * 网站站点
     */
    private volatile String webSiteCache;

    /**
     * 管理员邮箱
     */
    private volatile String adminMailCache;

    /**
     * 注册功能开关(三态:null 未加载,"1" 开启,"0" 关闭)
     */
    private volatile String registerEnabledCache;

    @Resource
    private SysConfigMapper sysConfigMapper;

    @Resource
    private SysUserMapper sysUserMapper;

    /**
     * 刷新缓存
     */
    @Override
    public void refreshCache() {
        synchronized (cacheLock) {
            emailConfigCache = null;
            webSiteCache = null;
            adminMailCache = null;
            registerEnabledCache = null;
        }
    }

    /**
     * 获取邮件配置
     *
     * @return 邮件配置
     */
    @Override
    public EmailConfigBo getEmailConfig() {
        EmailConfigBo local = emailConfigCache;
        if (local != null) {
            return local;
        }
        synchronized (cacheLock) {
            if (emailConfigCache != null) {
                return emailConfigCache;
            }
            emailConfigCache = loadEmailConfig();
            return emailConfigCache;
        }
    }

    /**
     * 获取脱敏后的邮件配置,密码字段以星号代替
     *
     * @return 脱敏后的邮件配置
     */
    @Override
    public EmailConfigBo getMaskedEmailConfig() {
        EmailConfigBo origin = getEmailConfig();
        if (origin == null) {
            return null;
        }
        EmailConfigBo masked = new EmailConfigBo();
        masked.setHostName(origin.getHostName());
        masked.setSslSmtpPort(origin.getSslSmtpPort());
        masked.setSmtpPort(origin.getSmtpPort());
        masked.setSsl(origin.getSsl());
        masked.setUsername(origin.getUsername());
        masked.setFrom(origin.getFrom());
        if (origin.getPassword() != null && !origin.getPassword().isEmpty()) {
            masked.setPassword(MASKED_PASSWORD);
        }
        return masked;
    }

    /**
     * 获取网站站点
     *
     * @return 网站站点
     */
    @Override
    public String getWebSite() {
        String local = webSiteCache;
        if (local != null) {
            return local;
        }
        synchronized (cacheLock) {
            if (webSiteCache != null) {
                return webSiteCache;
            }
            webSiteCache = loadSingleConfig("webSite", "站点地址");
            return webSiteCache;
        }
    }

    /**
     * 获取管理员邮箱
     *
     * @return 管理员邮箱
     */
    @Override
    public String getAdminMail() {
        String local = adminMailCache;
        if (local != null) {
            return local;
        }
        synchronized (cacheLock) {
            if (adminMailCache != null) {
                return adminMailCache;
            }
            adminMailCache = loadSingleConfig("adminMail", "管理员邮箱");
            return adminMailCache;
        }
    }

    /**
     * 获取站点配置
     *
     * @return 站点配置
     */
    @Override
    public SiteConfigBo getSiteConfig() {
        SiteConfigBo bo = new SiteConfigBo();
        bo.setWebSite(getWebSite());
        bo.setAdminMail(getAdminMail());
        bo.setRegisterEnabled(isRegisterEnabled());
        return bo;
    }

    /**
     * 判断系统是否已完成初始化
     * 以实际管理员账户存在为准，不信任SYS_CONFIG中的标记，防止篡改
     *
     * @return 是否已初始化
     */
    @Override
    public boolean isInitialized() {
        QueryWrapper userQuery = new QueryWrapper();
        userQuery.eq(SysUser::getAdmin, 1);
        long adminCount = sysUserMapper.selectCountByQuery(userQuery);
        boolean initialized = adminCount > 0;

        QueryWrapper configQuery = new QueryWrapper();
        configQuery.eq(SysConfig::getKey, "sys.initialized");
        List<SysConfig> configs = sysConfigMapper.selectListByQuery(configQuery);
        if (configs != null && !configs.isEmpty()) {
            SysConfig config = configs.get(0);
            String expected = initialized ? "1" : "0";
            if (!expected.equals(config.getValue())) {
                config.setValue(expected);
                sysConfigMapper.update(config);
            }
        }

        return initialized;
    }

    /**
     * 修改邮件配置(多条配置项原子更新)
     * 密码字段如收到掩码值则保留原密码,避免脱敏后写回覆盖
     */
    @Override
    @Transactional(rollbackFor = Exception.class)
    public void updateEmailConfig(UpdateEmailConfigDTO dto) {
        String password = dto.getPassword();
        if (MASKED_PASSWORD.equals(password)) {
            EmailConfigBo current = getEmailConfig();
            password = current == null ? "" : current.getPassword();
        }

        updateConfigByKey("mail.hostName", dto.getHostName());
        updateConfigByKey("mail.sslSmtpPort", String.valueOf(dto.getSslSmtpPort()));
        updateConfigByKey("mail.smtpPort", String.valueOf(dto.getSmtpPort()));
        updateConfigByKey("mail.ssl", dto.getSsl() != null && dto.getSsl() ? "1" : "0");
        updateConfigByKey("mail.userName", dto.getUserName());
        updateConfigByKey("mail.password", password);
        updateConfigByKey("mail.from", dto.getFrom());
        synchronized (cacheLock) {
            emailConfigCache = null;
        }
    }

    @Override
    @Transactional(rollbackFor = Exception.class)
    public void updateSiteConfig(UpdateSiteConfigDTO dto) {
        updateConfigByKey("webSite", dto.getWebSite());
        updateConfigByKey("adminMail", dto.getAdminMail());
        updateConfigByKey("sys.registerEnabled", dto.getRegisterEnabled() != null && dto.getRegisterEnabled() ? "1" : "0");
        synchronized (cacheLock) {
            webSiteCache = null;
            adminMailCache = null;
            registerEnabledCache = null;
        }
    }

    @Override
    public boolean isRegisterEnabled() {
        String local = registerEnabledCache;
        if (local != null) {
            return "1".equals(local);
        }
        synchronized (cacheLock) {
            if (registerEnabledCache != null) {
                return "1".equals(registerEnabledCache);
            }
            QueryWrapper qw = new QueryWrapper();
            qw.eq(SysConfig::getKey, "sys.registerEnabled");
            List<SysConfig> configs = sysConfigMapper.selectListByQuery(qw);
            registerEnabledCache = (configs == null || configs.isEmpty()) ? "0" : configs.get(0).getValue();
            return "1".equals(registerEnabledCache);
        }
    }

    private void updateConfigByKey(String key, String value) {
        QueryWrapper qw = new QueryWrapper();
        qw.eq(SysConfig::getKey, key);
        List<SysConfig> configs = sysConfigMapper.selectListByQuery(qw);
        if (configs != null && !configs.isEmpty()) {
            SysConfig config = configs.get(0);
            config.setValue(value);
            sysConfigMapper.update(config);
        }
    }

    private EmailConfigBo loadEmailConfig() {
        QueryWrapper queryWrapper = new QueryWrapper();
        queryWrapper.likeLeft(SysConfig::getKey, "mail");
        List<SysConfig> sysConfigs = sysConfigMapper.selectListByQuery(queryWrapper);
        if (sysConfigs == null || sysConfigs.isEmpty()) {
            log.warn("管理员未配置邮件参数，返回空邮箱配置信息。");
            return null;
        }

        Map<String, String> configMap = sysConfigs.stream()
                .filter(s -> s.getValue() != null)
                .collect(Collectors.toMap(SysConfig::getKey, SysConfig::getValue, (a, b) -> a));

        String hostName = configMap.get("mail.hostName");
        String sslSmtpPort = configMap.get("mail.sslSmtpPort");
        String smtpPort = configMap.get("mail.smtpPort");
        String username = configMap.get("mail.userName");
        String password = configMap.get("mail.password");
        String from = configMap.get("mail.from");

        if (isAnyBlank(hostName, sslSmtpPort, smtpPort, username, password, from)) {
            log.warn("管理员邮件参数不完整，返回空邮箱配置信息。");
            return null;
        }

        EmailConfigBo bo = new EmailConfigBo();
        bo.setHostName(hostName);
        try {
            bo.setSslSmtpPort(Integer.parseInt(sslSmtpPort));
            bo.setSmtpPort(Integer.parseInt(smtpPort));
        } catch (NumberFormatException e) {
            log.warn("邮件端口配置格式错误", e);
            return null;
        }
        bo.setSsl(Objects.equals(configMap.get("mail.ssl"), "1"));
        bo.setUsername(username);
        bo.setPassword(password);
        bo.setFrom(from);
        return bo;
    }

    private boolean isAnyBlank(String... values) {
        for (String v : values) {
            if (v == null || v.isEmpty()) {
                return true;
            }
        }
        return false;
    }

    private String loadSingleConfig(String key, String desc) {
        QueryWrapper queryWrapper = new QueryWrapper();
        queryWrapper.eq(SysConfig::getKey, key);
        List<SysConfig> sysConfigs = sysConfigMapper.selectListByQuery(queryWrapper);
        if (sysConfigs == null || sysConfigs.isEmpty()) {
            log.warn("管理员未配置{}参数。", desc);
            return null;
        }
        return sysConfigs.get(0).getValue();
    }
}
