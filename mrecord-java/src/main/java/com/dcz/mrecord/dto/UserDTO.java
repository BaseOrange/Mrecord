package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 用户注册、登录 DTO
 *
 * @author dcz
 * @since 2026/04/11
 */
@Data
public class UserDTO {
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
     * 重置密码token
     */
    private String rePasswordToken;
}
