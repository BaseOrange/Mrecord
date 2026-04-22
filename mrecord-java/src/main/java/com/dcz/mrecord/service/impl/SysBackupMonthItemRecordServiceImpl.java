package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.backup.SysBackupMonthItemRecord;
import com.dcz.mrecord.mapper.backup.SysBackupMonthItemRecordMapper;
import com.dcz.mrecord.service.SysBackupMonthItemRecordService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 月度财务明细项备份服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysBackupMonthItemRecordServiceImpl extends ServiceImpl<SysBackupMonthItemRecordMapper, SysBackupMonthItemRecord> implements SysBackupMonthItemRecordService {

    /**
     * 备份
     *
     * @param bookId 账簿ID
     */
    @Override
    public void backup(String bookId) {
        this.mapper.backupByBookId(bookId);
    }
}
