package com.dcz.mrecord.config;

import com.dcz.mrecord.entity.SysConfig;
import com.dcz.mrecord.mapper.SysConfigMapper;
import com.mybatisflex.core.query.QueryWrapper;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;
import lombok.Getter;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.annotation.Configuration;

import java.util.List;
import java.util.UUID;

/**
 * Mr配置
 * 从SYS_CONFIG表加载，仅启动时读取一次，修改后需重启服务生效
 *
 * @author dcz
 * @since 2026/04/13
 */
@Slf4j
@Getter
@Configuration
public class MrConf {

    @Resource
    private SysConfigMapper sysConfigMapper;

    private String jwtSecret;
    private Long jwtExpire;
    private String resetPwdTokenSecret;
    private String activateTokenSecret;

    @PostConstruct
    public void init() {
        this.jwtSecret = loadOrGenerate("mr.jwtSecret");
        String expireStr = loadConfigValue("mr.jwtExpire");
        this.jwtExpire = (expireStr != null && !expireStr.isEmpty()) ? Long.parseLong(expireStr) : 604800000L;
        this.resetPwdTokenSecret = loadOrGenerate("mr.resetPwdTokenSecret");
        this.activateTokenSecret = loadOrGenerate("mr.activateTokenSecret");
        log.info("已从SYS_CONFIG表加载安全配置项");
    }

    /**
     * 加载配置值，若为空则自动生成UUID并回写数据库
     */
    private String loadOrGenerate(String key) {
        String value = loadConfigValue(key);
        if (value != null && !value.isEmpty()) {
            return value;
        }
        value = UUID.randomUUID().toString().replace("-", "");
        updateConfigValue(key, value);
        log.info("配置项 {} 未设置，已自动生成并保存到数据库", key);
        return value;
    }

    private String loadConfigValue(String key) {
        QueryWrapper qw = new QueryWrapper();
        qw.eq(SysConfig::getKey, key);
        List<SysConfig> configs = sysConfigMapper.selectListByQuery(qw);
        if (configs == null || configs.isEmpty()) {
            return null;
        }
        return configs.get(0).getValue();
    }

    private void updateConfigValue(String key, String value) {
        QueryWrapper qw = new QueryWrapper();
        qw.eq(SysConfig::getKey, key);
        List<SysConfig> configs = sysConfigMapper.selectListByQuery(qw);
        if (configs != null && !configs.isEmpty()) {
            SysConfig config = configs.get(0);
            config.setValue(value);
            sysConfigMapper.update(config);
        }
    }
}
