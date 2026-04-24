package com.dcz.mrecord.service;

import com.dcz.mrecord.entity.SysExportTask;

/**
 * 导出任务异步执行服务
 *
 * @author dcz
 * @since 2026/04/21
 */
public interface ExportTaskExecutorService {

    /**
     * 执行账簿数据导出任务（异步）
     *
     * @param taskId         任务ID
     * @param userId         用户ID
     * @param bookId         账簿ID（可选）
     * @param startYearMonth 起始年月yyyyMM（可选）
     * @param endYearMonth   结束年月yyyyMM（可选）
     */
    SysExportTask executeExport(String taskId, String userId, String bookId, String startYearMonth, String endYearMonth);
}
