package com.dcz.mrecord.service.impl;

import cn.hutool.core.date.DateUtil;
import cn.hutool.core.util.StrUtil;
import cn.hutool.poi.excel.ExcelUtil;
import cn.hutool.poi.excel.ExcelWriter;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.constant.TempItemTypeConst;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.dcz.mrecord.entity.FinMonthRecord;
import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.entity.SysExportTask;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.exception.MrecordException;
import com.dcz.mrecord.mapper.FinTemplateItemMapper;
import com.dcz.mrecord.service.EmailService;
import com.dcz.mrecord.service.ExportTaskExecutorService;
import com.dcz.mrecord.service.FinBookService;
import com.dcz.mrecord.service.FinMonthItemRecordService;
import com.dcz.mrecord.service.FinMonthRecordService;
import com.dcz.mrecord.service.SysExportTaskService;
import com.dcz.mrecord.service.SysUserService;
import com.mybatisflex.core.query.QueryWrapper;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Service;

import java.io.File;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.function.Function;
import java.util.stream.Collectors;

/**
 * 导出任务异步执行服务实现
 *
 * @author dcz
 * @since 2026/04/21
 */
@Slf4j
@Service
public class ExportTaskExecutorServiceImpl implements ExportTaskExecutorService {

    @Resource
    private SysExportTaskService sysExportTaskService;
    @Resource
    private SysUserService sysUserService;
    @Resource
    private FinBookService finBookService;
    @Resource
    private FinMonthRecordService finMonthRecordService;
    @Resource
    private FinMonthItemRecordService finMonthItemRecordService;
    @Resource
    private FinTemplateItemMapper finTemplateItemMapper;
    @Resource
    private EmailService emailService;

    /**
     * 执行账簿数据导出任务（异步）
     * <p>整体流程：更新任务状态 → 查询账簿数据 → 生成Excel文件 → 更新任务结果 → 邮件通知用户</p>
     * <p>注意：该方法通过 {@code @Async} 异步执行，不可在同类中直接调用，否则代理失效</p>
     *
     * @param taskId         导出任务ID
     * @param userId         操作用户ID
     * @param bookId         账簿ID，为空则导出用户全部账簿
     * @param startYearMonth 起始年月，格式yyyyMM，为空则不限起始
     * @param endYearMonth   结束年月，格式yyyyMM，为空则不限结束
     */
    @Override
    @Async("exportTaskExecutor")
    public SysExportTask executeExport(String taskId, String userId, String bookId, String startYearMonth, String endYearMonth) {
        // 更新状态为执行中
        SysExportTask runningTask = new SysExportTask();
        runningTask.setId(taskId);
        runningTask.setStatus("RUN");
        sysExportTaskService.updateById(runningTask);

        File excelFile = null;
        try {
            SysUser user = sysUserService.getById(userId);
            if (user == null || StrUtil.isBlank(user.getEmail())) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "用户邮箱未配置，无法发送导出文件");
            }

            List<FinBook> books = getExportBooks(userId, bookId);
            if (books.isEmpty()) {
                throw new MrecordException(ResCode.PARAM_ERROR.getCode(), "无可导出的账簿数据");
            }

            String fileName = "账簿导出_" + DateUtil.format(new Date(), "yyyyMMddHHmmss") + ".xlsx";
            File exportDir = new File(System.getProperty("java.io.tmpdir"), "mrecord/export");
            if (!exportDir.exists()) {
                exportDir.mkdirs();
            }
            excelFile = new File(exportDir, fileName);

            try (ExcelWriter writer = ExcelUtil.getWriter(excelFile.getAbsolutePath())) {
                int sheetIndex = 0;
                for (FinBook book : books) {
                    if (sheetIndex > 0) {
                        writer.getWorkbook().createSheet(book.getBookName());
                    }
                    writer.setSheet(sheetIndex);
                    if (sheetIndex == 0) {
                        writer.renameSheet(book.getBookName());
                    }

                    writeBookData(writer, book, startYearMonth, endYearMonth);
                    sheetIndex++;
                }
            }

            SysExportTask successTask = new SysExportTask();
            successTask.setId(taskId);
            successTask.setStatus("SUCCESS");
            successTask.setFileName(fileName);
            sysExportTaskService.updateById(successTask);

            Map<String, String> params = new HashMap<>();
            params.put("MR-UserName", user.getNickname());
            params.put("MR-FileName", fileName);
            emailService.sendExportSuccessEmail(user.getEmail(), params, excelFile);

        } catch (Exception e) {
            log.error("导出任务执行失败, taskId={}", taskId, e);
            SysExportTask failTask = new SysExportTask();
            failTask.setId(taskId);
            failTask.setStatus("FAIL");
            failTask.setFailReason(StrUtil.sub(e.getMessage(), 0, 500));
            sysExportTaskService.updateById(failTask);

            if (excelFile != null && excelFile.exists()) {
                excelFile.delete();
            }
        }
        return runningTask;
    }

    /**
     * 获取待导出的账簿列表
     * <p>指定bookId时校验归属权限并返回单个账簿；未指定时返回用户所有账簿</p>
     *
     * @param userId 操作用户ID
     * @param bookId 账簿ID，为空则查询用户全部账簿
     * @return 待导出的账簿列表
     * @throws MrecordException 账簿不存在或无权限时抛出
     */
    private List<FinBook> getExportBooks(String userId, String bookId) {
        if (StrUtil.isNotBlank(bookId)) {
            FinBook book = finBookService.getById(bookId);
            if (book == null || !Objects.equals(book.getUserId(), userId)) {
                throw new MrecordException(ResCode.NO_PERMISSION.getCode(), "账簿不存在或无权限");
            }
            return Collections.singletonList(book);
        }
        QueryWrapper qw = QueryWrapper.create();
        qw.eq(FinBook::getUserId, userId);
        return finBookService.list(qw);
    }

    /**
     * 写入单个账簿的数据到指定Sheet页
     * <p>表头结构：统计月份 | 总资产 | 总负债 | 净资产 | 环比 | 同比 | 各资产明细项... | 各负债明细项...</p>
     * <p>数据行按年月升序排列，每月一行，月份间不空行</p>
     *
     * @param writer         Excel写入器，当前已定位到目标Sheet
     * @param book           账簿实体
     * @param startYearMonth 起始年月，格式yyyyMM，为空则不限起始
     * @param endYearMonth   结束年月，格式yyyyMM，为空则不限结束
     */
    private void writeBookData(ExcelWriter writer, FinBook book, String startYearMonth, String endYearMonth) {
        // 查询模板项，按类型和排序分组
        QueryWrapper tempQw = QueryWrapper.create();
        tempQw.eq(FinTemplateItem::getBookId, book.getId());
        List<FinTemplateItem> templateItems = finTemplateItemMapper.selectListByQuery(tempQw);
        if (templateItems == null || templateItems.isEmpty()) {
            throw new MrecordException(ResCode.FIN_ITEM_TEMP_IS_NOT);
        }

        List<FinTemplateItem> assetItems = templateItems.stream()
                .filter(t -> t.getItemType() != null && TempItemTypeConst.ASSET.intValue() == t.getItemType())
                .sorted(Comparator.comparingInt(t -> parseSort(t.getSort())))
                .toList();
        List<FinTemplateItem> liabilityItems = templateItems.stream()
                .filter(t -> t.getItemType() != null && TempItemTypeConst.LIABILITY.intValue() == t.getItemType())
                .sorted(Comparator.comparingInt(t -> parseSort(t.getSort())))
                .toList();

        // 查询月度汇总并排序
        QueryWrapper recordQw = QueryWrapper.create();
        recordQw.eq(FinMonthRecord::getBookId, book.getId());
        recordQw.orderBy(FinMonthRecord::getYear, true).orderBy(FinMonthRecord::getMonth, true);
        List<FinMonthRecord> monthRecords = finMonthRecordService.list(recordQw);
        monthRecords = filterByYearMonth(monthRecords, startYearMonth, endYearMonth, FinMonthRecord::getYear, FinMonthRecord::getMonth);

        // 查询明细
        QueryWrapper itemQw = QueryWrapper.create();
        itemQw.eq(FinMonthItemRecord::getBookId, book.getId());
        List<FinMonthItemRecord> itemRecords = finMonthItemRecordService.list(itemQw);
        itemRecords = filterByYearMonth(itemRecords, startYearMonth, endYearMonth, FinMonthItemRecord::getYear, FinMonthItemRecord::getMonth);

        Map<String, List<FinMonthItemRecord>> itemMap = itemRecords.stream()
                .collect(Collectors.groupingBy(r -> String.format("%04d%02d", r.getYear(), r.getMonth())));

        // 构建表头
        List<String> headList = new ArrayList<>();
        headList.add("统计月份");
        headList.add("总资产");
        headList.add("总负债");
        headList.add("净资产");
        headList.add("环比");
        headList.add("同比");
        for (FinTemplateItem item : assetItems) {
            headList.add(item.getItemName());
        }
        for (FinTemplateItem item : liabilityItems) {
            headList.add(item.getItemName());
        }

        writer.writeHeadRow(headList);

        // 写入数据行
        for (FinMonthRecord record : monthRecords) {
            List<Object> row = new ArrayList<>();
            row.add(String.format("%04d-%02d", record.getYear(), record.getMonth()));
            row.add(record.getTotalAsset());
            row.add(record.getTotalLiability());
            row.add(record.getNetAsset());
            row.add(formatPercent(record.getMonthOnMonth()));
            row.add(formatPercent(record.getYearOnYear()));

            String ym = String.format("%04d%02d", record.getYear(), record.getMonth());
            List<FinMonthItemRecord> monthItems = itemMap.getOrDefault(ym, Collections.emptyList());
            Map<String, BigDecimal> valueMap = monthItems.stream()
                    .collect(Collectors.toMap(FinMonthItemRecord::getTemplateItemId, FinMonthItemRecord::getItemValue, (a, b) -> a));

            for (FinTemplateItem item : assetItems) {
                row.add(valueMap.getOrDefault(item.getId(), BigDecimal.ZERO));
            }
            for (FinTemplateItem item : liabilityItems) {
                row.add(valueMap.getOrDefault(item.getId(), BigDecimal.ZERO));
            }

            writer.writeRow(row);
        }
    }

    /**
     * 按年月范围过滤记录（通用泛型方法）
     *
     * @param records        待过滤的记录列表
     * @param startYearMonth 起始年月（含），格式yyyyMM，为空则不限起始
     * @param endYearMonth   结束年月（含），格式yyyyMM，为空则不限结束
     * @param yearGetter     年份提取函数
     * @param monthGetter    月份提取函数
     * @param <T>            记录类型
     * @return 过滤后的记录列表
     */
    private <T> List<T> filterByYearMonth(List<T> records, String startYearMonth, String endYearMonth,
                                          Function<T, Integer> yearGetter, Function<T, Integer> monthGetter) {
        return records.stream().filter(r -> {
            String ym = String.format("%04d%02d", yearGetter.apply(r), monthGetter.apply(r));
            if (StrUtil.isNotBlank(startYearMonth) && ym.compareTo(startYearMonth) < 0) {
                return false;
            }
            return !StrUtil.isNotBlank(endYearMonth) || ym.compareTo(endYearMonth) <= 0;
        }).collect(Collectors.toList());
    }

    /**
     * 解析排序字段为整数
     *
     * @param sort 排序值字符串
     * @return 排序值，解析失败时返回 {@link Integer#MAX_VALUE}
     */
    private int parseSort(String sort) {
        try {
            return Integer.parseInt(sort);
        } catch (NumberFormatException e) {
            return Integer.MAX_VALUE;
        }
    }

    /**
     * 格式化百分比数值
     *
     * @param value 百分比数值，如 12.50 表示 12.50%
     * @return 格式化后的百分比字符串，为空时返回 "-"
     */
    private String formatPercent(BigDecimal value) {
        if (value == null) {
            return "-";
        }
        return value + "%";
    }
}
