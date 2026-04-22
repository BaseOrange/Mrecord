package com.dcz.mrecord.service;

import com.dcz.mrecord.entity.backup.SysBackupBook;
import com.mybatisflex.core.service.IService;

/**
 * 财务账簿备份服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface SysBackupBookService extends IService<SysBackupBook> {
    /**
     * 备份
     *
     * @param bookId 账簿ID
     */
    void backup(String bookId);
}
