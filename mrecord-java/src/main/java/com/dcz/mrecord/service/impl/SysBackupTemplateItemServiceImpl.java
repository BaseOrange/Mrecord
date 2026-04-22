package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.backup.SysBackupTemplateItem;
import com.dcz.mrecord.mapper.backup.SysBackupTemplateItemMapper;
import com.dcz.mrecord.service.SysBackupTemplateItemService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 记账模板明细备份服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysBackupTemplateItemServiceImpl extends ServiceImpl<SysBackupTemplateItemMapper, SysBackupTemplateItem> implements SysBackupTemplateItemService {
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
