package com.dcz.mrecord.dto;

import lombok.Data;

import java.io.Serializable;

/**
 * 用户注册、登录 DTO
 *
 * @author dcz
 * @since 2026/04/11
 */
@Data
public class UserDTO implements Serializable {

    /**
     * id
     */
    private String id;

    /**
     * 邮箱
     */
    private String email;

    /**
     * 密码
     */
    private String password;

    /**
     * 昵称
     */
    private String nickname;

    /**
     * 邮件提醒功能是否启用（0-关闭，1-开启）
     */
    private Integer remindEnabled;

    /**
     * 月度提醒日期（取值1-31，无对应日期取月末）
     */
    private Integer remindDay;

    /**
     * 重置密码token
     */
    private String rePasswordToken;
}
