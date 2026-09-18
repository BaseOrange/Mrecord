package com.dcz.mrecord.service;

import com.dcz.mrecord.bo.EmailConfigBo;
import com.dcz.mrecord.bo.MailParamsBO;

import java.util.List;

/**
 * 邮件服务
 *
 * @author dcz
 * @since 2026/04/11
 */
public interface EmailService {

    /**
     * 发送找回密码邮件
     *
     * @param params 邮件参数
     */
    void sendRetrievePasswordEmail(MailParamsBO params) throws Exception;

    /**
     * 发送注册成功邮件
     *
     * @param params 邮件参数
     */
    void sendRegisterSuccessEmail(MailParamsBO params);

    /**
     * 发送账户激活邮件
     *
     * @param params 邮件参数
     */
    void sendActivateAccountEmail(MailParamsBO params);

    /**
     * 发送月报邮件
     *
     * @param paramsList 邮件参数集合
     */
    void sendMonthReportEmail(List<MailParamsBO> paramsList);

    /**
     * 发送年度总结邮件
     *
     * <p>原方法 {@code sendNewYearReminderEmail} 语义为「新财年提醒」，产品决策不做新财年功能，
     * 改为每年 1 月 1 日 08:08 发送上一年度的财务总结（对应 Rust 端
     * {@code EmailService::send_year_summary_email} 与 {@code YearlySummaryTask}）。</p>
     *
     * @param paramsList 邮件参数集合
     */
    void sendYearSummaryEmail(List<MailParamsBO> paramsList);

    /**
     * 发送账簿导出完成邮件
     *
     * @param to         接收者邮箱
     * @param params     邮件参数
     * @param attachment 附件文件
     */
    void sendExportSuccessEmail(String to, java.util.Map<String, String> params, java.io.File attachment);

    /**
     * 发送测试邮件
     *
     * @param config 邮件配置（使用传入配置而非数据库配置）
     * @param to     收件人邮箱
     */
    void sendTestEmail(EmailConfigBo config, String to);
}
