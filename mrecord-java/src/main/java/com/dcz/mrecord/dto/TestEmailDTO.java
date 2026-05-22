package com.dcz.mrecord.dto;

import lombok.Data;

import java.io.Serializable;

/**
 * 测试邮件发送
 *
 * @author dcz
 * @since 2026/05/22
 */
@Data
public class TestEmailDTO implements Serializable {

    /**
     * SMTP服务器
     */
    private String hostName;

    /**
     * SSL-SMTP服务器端口
     */
    private Integer sslSmtpPort;

    /**
     * SMTP服务器端口
     */
    private Integer smtpPort;

    /**
     * 是否开启SSL
     */
    private Boolean ssl;

    /**
     * 邮箱用户名
     */
    private String userName;

    /**
     * 邮箱授权码
     */
    private String password;

    /**
     * 发送邮箱地址
     */
    private String from;

    /**
     * 测试收件邮箱
     */
    private String testTo;
}