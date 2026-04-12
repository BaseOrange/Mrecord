package com.dcz.mrecord.service;

import com.dcz.mrecord.bo.MailParamsBO;
import org.springframework.stereotype.Service;

import java.util.List;

/**
 * 邮件服务
 *
 * @author dcz
 * @since 2026/04/11
 */
@Service
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
     * 发送月报邮件
     *
     * @param paramsList 邮件参数集合
     */
    void sendMonthReportEmail(List<MailParamsBO> paramsList);

    /**
     * 送新财年总结邮件
     *
     * @param paramsList 邮件参数集合
     */
    void sendNewYearReminderEmail(List<MailParamsBO> paramsList);
}
