package com.dcz.mrecord.dto;

import lombok.Data;

import java.io.Serializable;

/**
 * 修改密码 DTO
 *
 * @author dcz
 * @since 2026/05/09
 */
@Data
public class ChangePasswordDTO implements Serializable {

    /**
     * 旧密码
     */
    private String oldPassword;

    /**
     * 新密码
     */
    private String newPassword;
}
