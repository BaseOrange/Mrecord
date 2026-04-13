package com.dcz.mrecord.config;

import lombok.Data;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Configuration;

/**
 * Mr配置
 *
 * @author dcz
 * @since 2026/04/13
 */
@Data
@Configuration
public class MrConf {
    /**
     * jwt密钥
     */
    @Value("${mr.jwtSecret}")
    private String jwtSecret;

    /**
     * jwt过期时间
     */
    @Value("${mr.jwtExpire}")
    private Long jwtExpire;

    /**
     * 重置密码令牌密钥
     */
    @Value("${mr.resetPwdTokenSecret}")
    private String resetPwdTokenSecret;
}
