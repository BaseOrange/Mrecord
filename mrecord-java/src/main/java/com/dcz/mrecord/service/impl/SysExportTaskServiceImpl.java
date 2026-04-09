package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.SysExportTask;
import com.dcz.mrecord.mapper.SysExportTaskMapper;
import com.dcz.mrecord.service.SysExportTaskService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 异步导出任务服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysExportTaskServiceImpl extends ServiceImpl<SysExportTaskMapper, SysExportTask> implements SysExportTaskService {
}
