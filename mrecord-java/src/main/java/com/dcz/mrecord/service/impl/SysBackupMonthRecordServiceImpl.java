package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.backup.SysBackupMonthRecord;
import com.dcz.mrecord.mapper.backup.SysBackupMonthRecordMapper;
import com.dcz.mrecord.service.SysBackupMonthRecordService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 月度财务汇总备份服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysBackupMonthRecordServiceImpl extends ServiceImpl<SysBackupMonthRecordMapper, SysBackupMonthRecord> implements SysBackupMonthRecordService {
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
