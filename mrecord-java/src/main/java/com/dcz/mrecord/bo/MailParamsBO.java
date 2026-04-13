package com.dcz.mrecord.bo;

import cn.hutool.core.date.DateUtil;
import lombok.Data;

import java.time.LocalDateTime;
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
    private String to;

    /**
     * 用户名
     */
    private String userName;

    /**
     * 用户邮箱
     */
    private String userEmail;

    /**
     * 注册时间
     */
    private String registerDate = DateUtil.format(LocalDateTime.now(), "yyyy-MM-dd HH:mm");

    /**
     * 当前年月
     */
    private String currYearMonth;

    /**
     * 当前年
     */
    private String currYear = DateUtil.format(LocalDateTime.now(), "yyyy");

    /**
     * 网站地址
     */
    private String webSite;

    /**
     * 忘记密码找回地址
     */
    private String repassword;

    /**
     * 管理员邮箱
     */
    private String adminMail;

    /**
     * 获取邮件参数
     *
     * @return 邮件参数
     */
    public Map<String, String> getParams() {
        return Map.of(
                "MR-UserName", getUserName(),
                "MR-UserEmail", getUserEmail(),
                "MR-YearMonth", getCurrYearMonth(),
                "MR-Year", getCurrYear(),
                "MR-WebSite", getWebSite(),
                "MR-AdminMail", getAdminMail(),
                "MR-Repassword", getRepassword(),
                "MR-RegisterDate", getRegisterDate()
        );
    }
}
