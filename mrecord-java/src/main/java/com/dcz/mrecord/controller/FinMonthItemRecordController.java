package com.dcz.mrecord.controller;

import com.dcz.mrecord.service.FinMonthItemRecordService;
import jakarta.annotation.Resource;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 月度财务明细项控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@RestController
@RequestMapping("/monthItemRecord")
public class FinMonthItemRecordController {
    @Resource
    private FinMonthItemRecordService finMonthItemRecordService;
}
