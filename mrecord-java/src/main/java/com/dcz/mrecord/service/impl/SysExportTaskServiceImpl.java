package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import com.dcz.mrecord.entity.SysExportTask;
import com.dcz.mrecord.mapper.SysExportTaskMapper;
import com.dcz.mrecord.service.SysExportTaskService;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

import java.util.List;

/**
 * 异步导出任务服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysExportTaskServiceImpl extends ServiceImpl<SysExportTaskMapper, SysExportTask> implements SysExportTaskService {

    @Override
    public String createExportTask(String userId, String bookId, String startYearMonth, String endYearMonth) {
        SysExportTask task = new SysExportTask();
        task.setId(IdUtil.simpleUUID());
        task.setUserId(userId);
        task.setBookId(bookId);
        task.setStartYearMonth(startYearMonth);
        task.setEndYearMonth(endYearMonth);
        task.setStatus("WAIT");
        this.save(task);
        return task.getId();
    }

    @Override
    public List<SysExportTask> listMyTasks(String userId) {
        QueryWrapper qw = QueryWrapper.create();
        qw.eq(SysExportTask::getUserId, userId);
        qw.orderBy(SysExportTask::getCreateTime, false);
        return this.list(qw);
    }
}
