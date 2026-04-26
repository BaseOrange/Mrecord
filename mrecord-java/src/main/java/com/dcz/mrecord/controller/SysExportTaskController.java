package com.dcz.mrecord.controller;

import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.dto.ExportBookDTO;
import com.dcz.mrecord.dto.PageInfoDTO;
import com.dcz.mrecord.entity.SysExportTask;
import com.dcz.mrecord.service.ExportTaskExecutorService;
import com.dcz.mrecord.service.SysExportTaskService;
import com.mybatisflex.core.paginate.Page;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

/**
 * 异步导出任务控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@RestController
@RequestMapping("/exportTask")
public class SysExportTaskController {

    @Resource
    private SysExportTaskService sysExportTaskService;
    @Resource
    private ExportTaskExecutorService exportTaskExecutorService;

    /**
     * 发起账簿数据导出任务
     *
     * @param dto 导出参数
     * @return 任务ID
     */
    @PostMapping("/export")
    public Result<SysExportTask> export(@RequestBody ExportBookDTO dto) {
        String userId = UserContext.getUserId();
        log.info("发起账簿数据导出任务[/exportTask/export]请求传参：{} {}", userId, dto);
        String taskId = sysExportTaskService.createExportTask(userId, dto.getBookId(), dto.getStartYearMonth(), dto.getEndYearMonth());
        SysExportTask sysExportTask = exportTaskExecutorService.executeExport(taskId, userId, dto.getBookId(), dto.getStartYearMonth(), dto.getEndYearMonth());
        return Result.success(sysExportTask);
    }

    /**
     * 查询当前用户的导出任务列表
     *
     * @return 任务列表
     */
    @PostMapping("/list")
    public Result<Page<SysExportTask>> list(@RequestBody PageInfoDTO pageInfoDTO) {
        String userId = UserContext.getUserId();
        log.info("查询当前用户的导出任务列表[/exportTask/list]请求传参：{} {}", userId, pageInfoDTO);
        return Result.success(sysExportTaskService.listMyTasks(userId, pageInfoDTO));
    }
}
