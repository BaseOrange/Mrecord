package com.dcz.mrecord.service;

import com.dcz.mrecord.entity.backup.SysBackupMonthRecord;
import com.mybatisflex.core.service.IService;

/**
 * 月度财务汇总备份服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface SysBackupMonthRecordService extends IService<SysBackupMonthRecord> {
    /**
     * 备份
     *
     * @param bookId 账簿ID
     */
    void backup(String bookId);
}
