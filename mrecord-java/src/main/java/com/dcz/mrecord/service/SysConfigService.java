package com.dcz.mrecord.service;

import com.dcz.mrecord.bo.EmailConfigBo;
import com.dcz.mrecord.bo.SiteConfigBo;
import com.dcz.mrecord.dto.UpdateEmailConfigDTO;
import com.dcz.mrecord.dto.UpdateSiteConfigDTO;
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

    /**
     * 判断系统是否已完成初始化（存在管理员账户）
     *
     * @return 是否已初始化
     */
    boolean isInitialized();

    /**
     * 获取站点配置
     *
     * @return 站点配置
     */
    SiteConfigBo getSiteConfig();

    /**
     * 修改邮件配置
     *
     * @param dto 邮件配置
     */
    void updateEmailConfig(UpdateEmailConfigDTO dto);

    /**
     * 修改站点配置（站点地址、管理员邮箱、注册开关）
     *
     * @param dto 站点配置
     */
    void updateSiteConfig(UpdateSiteConfigDTO dto);

    /**
     * 判断是否开启注册功能
     *
     * @return 是否开启注册
     */
    boolean isRegisterEnabled();
}
