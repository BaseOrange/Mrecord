package com.dcz.mrecord.controller;

import com.dcz.mrecord.service.FinMonthRecordService;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 月度财务汇总控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/monthRecord")
public class FinMonthRecordController {
    @Resource
    private FinMonthRecordService finMonthRecordService;
}
