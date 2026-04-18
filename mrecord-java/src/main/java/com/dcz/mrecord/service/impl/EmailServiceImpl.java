package com.dcz.mrecord.service.impl;

import cn.hutool.core.io.resource.ClassPathResource;
import com.dcz.mrecord.bo.EmailConfigBo;
import com.dcz.mrecord.bo.MailParamsBO;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.service.EmailService;
import com.dcz.mrecord.service.SysConfigService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.mail.DefaultAuthenticator;
import org.apache.commons.mail.EmailConstants;
import org.apache.commons.mail.EmailException;
import org.apache.commons.mail.HtmlEmail;
import org.springframework.stereotype.Service;
import org.springframework.util.FileCopyUtils;

import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Map;

/**
 * 邮件服务实现类
 *
 * @author dcz
 * @since 2026/04/11
 */
@Slf4j
@Service
public class EmailServiceImpl implements EmailService {

    @Resource
    private SysConfigService sysConfigService;

    /**
     * 发送找回密码邮件
     *
     * @param params 邮件参数
     */
    @Override
    public void sendRetrievePasswordEmail(MailParamsBO params) {
        try {
            HtmlEmail mailClient = getMailClient();
            if (mailClient == null) {
                throw new MrecordException(ResCode.BUSINESS_ERROR.getCode(), "管理员未配置邮箱参数，请联系管理员重置密码");
            }

            sendHtmlMail(mailClient, params.getTo(), "【MRecord｜月衡】密码重置", "mail/mr-password.html", params.getParams());
        } catch (MrecordException e) {
            throw e;
        } catch (Exception e) {
            log.error("重置密码邮件发送失败", e);
            throw new MrecordException(ResCode.ERROR.getCode(), "重置密码邮件发送失败，请联系管理员");
        }
    }

    /**
     * 发送注册成功邮件
     *
     * @param params 邮件参数
     */
    @Override
    public void sendRegisterSuccessEmail(MailParamsBO params) {
        try {
            HtmlEmail mailClient = getMailClient();
            sendHtmlMail(mailClient, params.getTo(), "【MRecord｜月衡】欢迎使用月衡", "mail/mr-register.html", params.getParams());
        } catch (Exception e) {
            log.error("注册欢迎邮件发送失败", e);
        }
    }

    /**
     * 发送月报邮件
     *
     * @param paramsList 邮件参数集合
     */
    @Override
    public void sendMonthReportEmail(List<MailParamsBO> paramsList) {
        HtmlEmail mailClient = null;

        for (MailParamsBO params : paramsList) {
            try {
                mailClient = mailClient == null ? getMailClient() : mailClient;
                sendHtmlMail(mailClient, params.getTo(), "【MRecord｜月衡】新月度记账提醒", "mail/mr-bookkeep.html", params.getParams());
            } catch (Exception e) {
                log.error("注册欢迎邮件发送失败", e);
            }
        }
    }

    /**
     * 送新财年总结邮件
     *
     * @param paramsList 邮件参数集合
     */
    @Override
    public void sendNewYearReminderEmail(List<MailParamsBO> paramsList) {
        HtmlEmail mailClient = null;

        for (MailParamsBO params : paramsList) {
            try {
                mailClient = mailClient == null ? getMailClient() : mailClient;
                sendHtmlMail(mailClient, params.getTo(), "【MRecord｜月衡】开启新的一年吧", "mail/mr-year.html", params.getParams());
            } catch (Exception e) {
                log.error("新财年提醒邮件发送失败", e);
            }
        }
    }

    /**
     * 发送HTML邮件
     *
     * @param to       接收者邮箱
     * @param subject  邮件主题
     * @param tempPath 邮件模板路径
     * @param params   邮件参数
     * @throws Exception 邮件发送异常
     */
    private void sendHtmlMail(HtmlEmail email, String to, String subject, String tempPath, Map<String, String> params) throws Exception {
        if (email == null) {
            log.warn("管理员未配置邮件参数，跳过邮件发送逻辑。");
            return;
        }

        email.addTo(to);
        email.setSubject(subject);
        email.setHtmlMsg(readTemplate(tempPath, params));
        email.send();
    }

    /**
     * 读取HTML模板并替换占位符
     *
     * @param templatePath HTML模板路径
     * @param params       占位符参数
     * @return 替换后的HTML字符串
     * @throws Exception 读取模板异常
     */
    private String readTemplate(String templatePath, Map<String, String> params) throws Exception {
        ClassPathResource resource = new ClassPathResource(templatePath);
        String html = FileCopyUtils.copyToString(new InputStreamReader(resource.getStream(), StandardCharsets.UTF_8));

        for (Map.Entry<String, String> entry : params.entrySet()) {
            html = html.replace("${" + entry.getKey() + "}", entry.getValue());
        }
        return html;
    }

    /**
     * 获取邮件客户端
     *
     * @return 邮件客户端
     * @throws EmailException 邮件客户端初始化异常
     */
    private HtmlEmail getMailClient() throws EmailException {
        EmailConfigBo emailConfig = sysConfigService.getEmailConfig();
        if (emailConfig == null) {
            return null;
        }

        HtmlEmail email = new HtmlEmail();
        email.setHostName(emailConfig.getHostName());
        email.setSmtpPort(emailConfig.getSmtpPort());
        Boolean ssl = emailConfig.getSsl();
        if (ssl != null) {
            email.setSSLOnConnect(ssl);
        }
        email.setAuthenticator(new DefaultAuthenticator(emailConfig.getUsername(), emailConfig.getPassword()));
        email.setFrom(emailConfig.getFrom());
        email.setCharset(EmailConstants.UTF_8);

        return email;
    }
}
