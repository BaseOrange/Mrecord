package com.dcz.mrecord.bo;

import lombok.Data;

/**
 * 邮件配置
 *
 * @author dcz
 * @since 2026/04/12
 */
@Data
public class EmailConfigBo {

    /**
     * 邮件服务器主机名
     */
    private String hostName;

    /**
     * 邮件服务器SMTP端口
     */
    private Integer smtpPort;

    /**
     * 是否使用SSL
     */
    private Boolean ssl;

    /**
     * 邮件服务器用户名
     */
    private String username;

    /**
     * 邮件服务器密码
     */
    private String password;

    /**
     * 邮件发送者
     */
    private String from;
}
