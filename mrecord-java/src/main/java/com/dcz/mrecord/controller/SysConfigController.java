package com.dcz.mrecord.controller;

import com.dcz.mrecord.service.SysConfigService;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 配置项控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/config")
public class SysConfigController {
    @Resource
    private SysConfigService sysConfigService;
}
