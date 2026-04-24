package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.dto.PageInfoDTO;
import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.entity.SysExportTask;
import com.dcz.mrecord.mapper.SysExportTaskMapper;
import com.dcz.mrecord.service.SysExportTaskService;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
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
    @Resource
    private SysExportTaskMapper sysExportTaskMapper;

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
    public Page<SysExportTask> listMyTasks(String userId, PageInfoDTO pageInfoDTO) {
        Page<SysExportTask> page = new Page<>(pageInfoDTO.getPageNum(), pageInfoDTO.getPageSize());
        QueryWrapper qw = QueryWrapper.create();
        qw.eq(SysExportTask::getUserId, userId);
        qw.orderBy(SysExportTask::getCreateTime, false);
        return sysExportTaskMapper.paginate(page, qw);
    }
}
