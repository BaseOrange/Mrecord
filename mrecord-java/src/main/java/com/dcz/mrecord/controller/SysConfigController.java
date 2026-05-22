package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.CheckAdmin;
import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.UpdateEmailConfigDTO;
import com.dcz.mrecord.dto.UpdateSiteConfigDTO;
import com.dcz.mrecord.service.SysConfigService;
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
}
