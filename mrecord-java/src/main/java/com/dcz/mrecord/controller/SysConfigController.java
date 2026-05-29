package com.dcz.mrecord.controller;

import com.dcz.mrecord.bo.EmailConfigBo;
import com.dcz.mrecord.bo.SiteConfigBo;
import com.dcz.mrecord.common.CheckAdmin;
import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.InitAdminDTO;
import com.dcz.mrecord.dto.TestEmailDTO;
import com.dcz.mrecord.dto.UpdateEmailConfigDTO;
import com.dcz.mrecord.dto.UpdateSiteConfigDTO;
import com.dcz.mrecord.service.EmailService;
import com.dcz.mrecord.service.SysConfigService;
import com.dcz.mrecord.service.SysUserService;
import com.dcz.mrecord.util.JwtUtil;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 配置项控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@RestController
@RequestMapping("/config")
public class SysConfigController {
    @Resource
    private SysConfigService sysConfigService;

    @Resource
    private SysUserService sysUserService;

    @Resource
    private EmailService emailService;

    @Resource
    private JwtUtil jwtUtil;

    @CheckAdmin
    @PostMapping("/refreshCache")
    public Result<String> refreshCache() {
        log.info("刷新配置项缓存[/config/refreshCache]请求");
        sysConfigService.refreshCache();
        return Result.success();
    }

    @PostMapping("/initialized")
    public Result<Boolean> initialized() {
        log.info("获取系统初始化状态[/config/initialized]请求");
        return Result.success(sysConfigService.isInitialized());
    }

    @PostMapping("/registerEnabled")
    public Result<Boolean> registerEnabled() {
        log.info("获取是否开启注册功能[/config/registerEnabled]请求");
        return Result.success(sysConfigService.isRegisterEnabled());
    }

    @CheckAdmin
    @PostMapping("/getEmailConfig")
    public Result<EmailConfigBo> getEmailConfig() {
        log.info("获取邮件配置[/config/getEmailConfig]请求");
        return Result.success(sysConfigService.getMaskedEmailConfig());
    }

    @CheckAdmin
    @PostMapping("/getSiteConfig")
    public Result<SiteConfigBo> getSiteConfig() {
        log.info("获取站点配置[/config/getSiteConfig]请求");
        return Result.success(sysConfigService.getSiteConfig());
    }

    @CheckAdmin
    @PostMapping("/updateEmailConfig")
    public Result<String> updateEmailConfig(@RequestBody UpdateEmailConfigDTO dto) {
        log.info("修改邮件配置[/config/updateEmailConfig]请求");
        sysConfigService.updateEmailConfig(dto);
        return Result.success();
    }

    @CheckAdmin
    @PostMapping("/updateSiteConfig")
    public Result<String> updateSiteConfig(@RequestBody UpdateSiteConfigDTO dto) {
        log.info("修改站点配置[/config/updateSiteConfig]请求");
        sysConfigService.updateSiteConfig(dto);
        return Result.success();
    }

    @PostMapping("/initAdmin")
    public Result<String> initAdmin(@RequestBody InitAdminDTO dto) {
        log.info("初始化管理员账户[/config/initAdmin]请求");
        String userId = sysUserService.initAdmin(dto);
        String token = jwtUtil.createToken(userId);
        return Result.success(token);
    }

    @CheckAdmin
    @PostMapping("/testEmail")
    public Result<String> testEmail(@RequestBody TestEmailDTO dto) {
        log.info("测试邮件发送[/config/testEmail]请求");
        EmailConfigBo config = new EmailConfigBo();
        config.setHostName(dto.getHostName());
        config.setSslSmtpPort(dto.getSslSmtpPort());
        config.setSmtpPort(dto.getSmtpPort());
        config.setSsl(dto.getSsl());
        config.setUsername(dto.getUserName());
        config.setPassword(dto.getPassword());
        config.setFrom(dto.getFrom());
        emailService.sendTestEmail(config, dto.getTestTo());
        return Result.success();
    }
}
