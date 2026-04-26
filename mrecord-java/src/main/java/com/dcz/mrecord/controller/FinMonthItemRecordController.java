package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.MonthItemDTO;
import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.dcz.mrecord.service.FinMonthItemRecordService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;
import java.util.Map;

/**
 * 月度财务账目控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@RestController
@RequestMapping("/monthItem")
public class FinMonthItemRecordController {
    @Resource
    private FinMonthItemRecordService finMonthItemRecordService;

    /**
     * 插入月度财务账目
     *
     * @param monthItemDTO 月度财务账目DTO
     * @return 插入结果
     */
    @PostMapping("/insertMonthItem")
    public Result<List<FinMonthItemRecord>> insertMonthItem(@RequestBody MonthItemDTO monthItemDTO) {
        log.info("插入月度财务账目[/monthItem/insertMonthItem]请求传参：{}", monthItemDTO);
        List<FinMonthItemRecord> resList = finMonthItemRecordService.insertMonthItemRecord(monthItemDTO);
        return Result.success(resList);
    }

    /**
     * 更新月度财务账目
     *
     * @param monthItemDTO 月度财务账目DTO
     * @return 更新结果
     */
    @PostMapping("/updateMonthItem")
    public Result<List<FinMonthItemRecord>> updateMonthItem(@RequestBody MonthItemDTO monthItemDTO) {
        log.info("更新月度财务账目[/monthItem/updateMonthItem]请求传参：{}", monthItemDTO);
        List<FinMonthItemRecord> resList = finMonthItemRecordService.updateMonthItemRecord(monthItemDTO);
        return Result.success(resList);
    }

    /**
     * 查询月度财务账目
     *
     * @param monthItemDTO 月度财务账目DTO
     * @return 查询结果
     */
    @PostMapping("/queryMonthItem")
    public Result<List<FinMonthItemRecord>> queryMonthItem(@RequestBody MonthItemDTO monthItemDTO) {
        log.info("查询月度财务账目[/monthItem/queryMonthItem]请求传参：{}", monthItemDTO);
        List<FinMonthItemRecord> resList = finMonthItemRecordService.queryByBookIdAndMonth(monthItemDTO);
        return Result.success(resList);
    }

    /**
     * 查询所有月度财务账目
     *
     * @param monthItemDTO 月度财务账目DTO
     * @return 查询结果
     */
    @PostMapping("/queryAll")
    public Result<Map<String, List<FinMonthItemRecord>>> queryAllMonthItem(@RequestBody MonthItemDTO monthItemDTO) {
        log.info("查询所有月度财务账目[/monthItem/queryAll]请求传参：{}", monthItemDTO);
        Map<String, List<FinMonthItemRecord>> resMap = finMonthItemRecordService.queryAllByBookId(monthItemDTO);
        return Result.success(resMap);
    }
}
