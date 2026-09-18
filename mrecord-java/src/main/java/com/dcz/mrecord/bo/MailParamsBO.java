package com.dcz.mrecord.bo;

import cn.hutool.core.date.DateUtil;
import cn.hutool.extra.spring.SpringUtil;
import com.dcz.mrecord.service.SysConfigService;
import lombok.Data;

import java.time.LocalDateTime;
import java.util.HashMap;
import java.util.Map;

/**
 * 邮件参数BO
 *
 * @author dcz
 * @since 2026/04/12
 */
@Data
public class MailParamsBO {

    /**
     * 接收者邮箱
     */
    private String to = "";

    /**
     * 用户名
     */
    private String userName = "";

    /**
     * 用户邮箱
     */
    private String userEmail = "";

    /**
     * 注册时间
     */
    private String registerDate = DateUtil.format(LocalDateTime.now(), "yyyy-MM-dd HH:mm");

    /**
     * 当前年月
     */
    private String currYearMonth = DateUtil.format(LocalDateTime.now(), "M");;

    /**
     * 当前年
     */
    private String currYear = DateUtil.format(LocalDateTime.now(), "yyyy");

    /**
     * 网站地址
     */
    private String webSite = "";

    /**
     * 忘记密码找回地址
     */
    private String repassword = "";

    /**
     * 账户激活地址
     */
    private String activateUrl = "";

    /**
     * 管理员邮箱
     */
    private String adminMail = "";

    // ==================== 年度总结邮件专用 ====================

    /**
     * 总结年份（上一年，如 "2025"）
     */
    private String summaryYear = "";

    /**
     * 记账月数
     */
    private String recordedMonths = "";

    /**
     * 账簿数量
     */
    private String bookCount = "";

    /**
     * 年度明细记录条数
     */
    private String itemCount = "";

    /**
     * 年末总资产
     */
    private String totalAsset = "";

    /**
     * 年末总负债
     */
    private String totalLiability = "";

    /**
     * 年末净资产
     */
    private String netAsset = "";

    /**
     * 年初净资产
     */
    private String netAssetStart = "";

    /**
     * 净资产变化金额（带符号）
     */
    private String netAssetChange = "";

    /**
     * 净资产变化率（带符号 %，年初为零无可比基准时为 "—"）
     */
    private String netAssetChangeRate = "";

    /**
     * 获取邮件参数
     *
     * @return 邮件参数
     */
    public Map<String, String> getParams() {
        SysConfigService configService = SpringUtil.getBean(SysConfigService.class);
        String site = configService.getWebSite();
        if (site != null) {
            webSite = site;
        }
        String mail = configService.getAdminMail();
        if (mail != null) {
            adminMail = mail;
        }

        Map<String, String> params = new HashMap<>();
        params.put("MR-UserName", getUserName());
        params.put("MR-UserEmail", getUserEmail());
        params.put("MR-YearMonth", getCurrYearMonth());
        params.put("MR-Year", getCurrYear());
        params.put("MR-WebSite", getWebSite());
        params.put("MR-AdminMail", getAdminMail());
        params.put("MR-Repassword", getRepassword());
        params.put("MR-ActivateUrl", getActivateUrl());
        params.put("MR-RegisterDate", getRegisterDate());
        // 年度总结占位符
        params.put("MR-SummaryYear", getSummaryYear());
        params.put("MR-RecordedMonths", getRecordedMonths());
        params.put("MR-BookCount", getBookCount());
        params.put("MR-ItemCount", getItemCount());
        params.put("MR-TotalAsset", getTotalAsset());
        params.put("MR-TotalLiability", getTotalLiability());
        params.put("MR-NetAsset", getNetAsset());
        params.put("MR-NetAssetStart", getNetAssetStart());
        params.put("MR-NetAssetChange", getNetAssetChange());
        params.put("MR-NetAssetChangeRate", getNetAssetChangeRate());
        return params;
    }
}
