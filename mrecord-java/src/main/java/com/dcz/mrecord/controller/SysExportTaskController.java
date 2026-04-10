package com.dcz.mrecord.controller;

import com.dcz.mrecord.service.SysExportTaskService;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 异步导出任务控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/exportTask")
public class SysExportTaskController {
    @Resource
    private SysExportTaskService sysExportTaskService;
}
