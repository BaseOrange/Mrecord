package com.dcz.mrecord.service;

import com.dcz.mrecord.entity.SysExportTask;
import com.mybatisflex.core.service.IService;

import java.util.List;

/**
 * 异步导出任务服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface SysExportTaskService extends IService<SysExportTask> {

    /**
     * 创建导出任务
     *
     * @param userId         用户ID
     * @param bookId         账簿ID（可选）
     * @param startYearMonth 起始年月（可选）
     * @param endYearMonth   结束年月（可选）
     * @return 任务ID
     */
    String createExportTask(String userId, String bookId, String startYearMonth, String endYearMonth);

    /**
     * 查询当前用户的导出任务列表（按创建时间倒序）
     *
     * @param userId 用户ID
     * @return 任务列表
     */
    List<SysExportTask> listMyTasks(String userId);
}
