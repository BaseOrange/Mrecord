package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.MonthRecordDTO;
import com.dcz.mrecord.entity.FinMonthRecord;
import com.dcz.mrecord.service.FinMonthRecordService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

/**
 * 月度财务汇总控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@RestController
@RequestMapping("/monthRecord")
public class FinMonthRecordController {
    @Resource
    private FinMonthRecordService finMonthRecordService;

    /**
     * 获取月度财务汇总
     *
     * @param monthRecordDTO 月度财务汇总参数
     * @return 月度财务汇总
     */
    @PostMapping("/getMonthRecord")
    public Result<FinMonthRecord> getMonthRecord(@RequestBody MonthRecordDTO monthRecordDTO) {
        log.info("获取月度财务汇总[/monthRecord/getMonthRecord]请求传参：{}", monthRecordDTO);
        FinMonthRecord monthRecord = finMonthRecordService.getMonthRecord(monthRecordDTO);
        return Result.success(monthRecord);
    }

    /**
     * 获取年度财务汇总列表
     *
     * @param monthRecordDTO 年度财务汇总参数
     * @return 年度财务汇总列表
     */
    @PostMapping("/getYearRecordList")
    public Result<List<FinMonthRecord>> getYearRecordList(@RequestBody MonthRecordDTO monthRecordDTO) {
        log.info("获取年度财务汇总列表[/monthRecord/getYearRecordList]请求传参：{}", monthRecordDTO);
        return Result.success(finMonthRecordService.getYearRecordList(monthRecordDTO));
    }
}
