package com.dcz.mrecord.dto;

import lombok.Data;

import java.io.Serializable;

/**
 * 修改站点配置 DTO
 *
 * @author dcz
 * @since 2026/05/22
 */
@Data
public class UpdateSiteConfigDTO implements Serializable {

    /**
     * 站点地址
     */
    private String webSite;

    /**
     * 管理员邮箱
     */
    private String adminMail;

    /**
     * 是否开启注册功能
     */
    private Boolean registerEnabled;
}
