package com.dcz.mrecord.service.impl;

import cn.hutool.core.util.IdUtil;
import com.dcz.mrecord.constant.ExportTaskStatusConst;
import com.dcz.mrecord.dto.PageInfoDTO;
import com.dcz.mrecord.entity.SysExportTask;
import com.dcz.mrecord.mapper.SysExportTaskMapper;
import com.dcz.mrecord.service.SysExportTaskService;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.query.QueryWrapper;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import jakarta.annotation.Resource;
import org.springframework.stereotype.Service;

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
        task.setStatus(ExportTaskStatusConst.WAIT);
        this.save(task);
        return task.getId();
    }

    @Override
    public Page<SysExportTask> listMyTasks(String userId, PageInfoDTO pageInfoDTO) {
        QueryWrapper qw = QueryWrapper.create()
                .select("t1.*", "b.MR_BOOK_NAME as bookName")
                .from("SYS_EXPORT_TASK").as("t1")
                .leftJoin("FIN_BOOK").as("b").on("t1.MR_BOOK_ID = b.MR_ID")
                .where("t1.MR_USER_ID = ?", userId)
                .orderBy("t1.MR_CREATE_TIME desc");
        return sysExportTaskMapper.paginate(pageInfoDTO.getPageNum(), pageInfoDTO.getPageSize(), qw);
    }
}
