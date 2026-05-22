package com.dcz.mrecord.dto;

import lombok.Data;

import java.io.Serializable;

/**
 * 初始化管理员账户
 *
 * @author dcz
 * @since 2026/05/22
 */
@Data
public class InitAdminDTO implements Serializable {

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
}