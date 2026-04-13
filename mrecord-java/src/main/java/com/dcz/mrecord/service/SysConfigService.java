package com.dcz.mrecord.service;

import com.dcz.mrecord.bo.EmailConfigBo;
import com.dcz.mrecord.entity.SysConfig;
import com.mybatisflex.core.service.IService;

/**
 * 配置项服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface SysConfigService extends IService<SysConfig> {
    /**
     * 刷新缓存
     */
    void refreshCache();

    /**
     * 获取邮件配置
     *
     * @return 邮件配置
     */
    EmailConfigBo getEmailConfig();

    /**
     * 获取网站地址
     *
     * @return 网站地址
     */
    String getWebSite();

    /**
     * 获取管理员邮箱
     *
     * @return 管理员邮箱
     */
    String getAdminMail();
}
