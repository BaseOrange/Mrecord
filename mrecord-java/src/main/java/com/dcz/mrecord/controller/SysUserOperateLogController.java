package com.dcz.mrecord.controller;

import com.dcz.mrecord.service.SysUserOperateLogService;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 用户操作审计日志控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/operateLog")
public class SysUserOperateLogController {
    @Resource
    private SysUserOperateLogService sysUserOperateLogService;
}
