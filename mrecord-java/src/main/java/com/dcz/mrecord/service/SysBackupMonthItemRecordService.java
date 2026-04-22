package com.dcz.mrecord.service;

import com.dcz.mrecord.entity.backup.SysBackupMonthItemRecord;
import com.mybatisflex.core.service.IService;

/**
 * 月度财务明细项备份服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface SysBackupMonthItemRecordService extends IService<SysBackupMonthItemRecord> {
    /**
     * 备份
     *
     * @param bookId 账簿ID
     */
    void backup(String bookId);
}
