package com.dcz.mrecord.config;

import com.dcz.mrecord.entity.BaseEntity;
import com.dcz.mrecord.interceptor.AuditFieldListener;
import com.mybatisflex.core.FlexGlobalConfig;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/**
 * MyBatisFlex配置类
 *
 * @author dcz
 * @since 2023/4/16 08:44
 */
@Configuration
public class MyBatisFlexConfig {
    /**
     * 配置全局的FlexGlobalConfig
     *
     * @param auditListener 审计字段监听器
     * @return FlexGlobalConfig
     */
    @Bean
    public FlexGlobalConfig flexGlobalConfig(AuditFieldListener auditListener) {
        FlexGlobalConfig config = FlexGlobalConfig.getDefaultConfig();

        // 全局注册插入/更新监听器
        config.registerInsertListener(auditListener, BaseEntity.class);
        config.registerUpdateListener(auditListener, BaseEntity.class);

        return config;
    }
}
