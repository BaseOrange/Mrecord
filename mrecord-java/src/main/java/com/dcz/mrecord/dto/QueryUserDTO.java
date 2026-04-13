package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 查询用户参数
 *
 * @author dcz
 * @since 2026/04/13
 */
@Data
public class QueryUserDTO extends PageInfoDTO {

    /**
     * 昵称
     */
    private String nickname;

    /**
     * 邮箱
     */
    private String email;

    /**
     * 状态
     */
    private Integer status;

    /**
     * 是否管理员
     */
    private Integer isAdmin;
}
