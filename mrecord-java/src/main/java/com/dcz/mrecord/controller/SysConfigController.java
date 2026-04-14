package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.CheckAdmin;
import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.service.SysConfigService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
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
}
