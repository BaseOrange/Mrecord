package com.dcz.mrecord.bo;

import lombok.Data;

/**
 * 站点配置
 *
 * @author dcz
 * @since 2026/05/22
 */
@Data
public class SiteConfigBo {

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