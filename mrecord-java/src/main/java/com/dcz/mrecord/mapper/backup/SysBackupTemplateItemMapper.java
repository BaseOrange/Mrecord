package com.dcz.mrecord.mapper.backup;

import com.dcz.mrecord.entity.backup.SysBackupTemplateItem;
import com.mybatisflex.core.BaseMapper;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;

/**
 * 记账模板明细备份Mapper
 *
 * @author dcz
 * @since 2026/04/07
 */
@Mapper
public interface SysBackupTemplateItemMapper extends BaseMapper<SysBackupTemplateItem> {

    /**
     * 根据账簿ID备份记账模板明细数据
     *
     * @param bookId 账簿ID
     */
    void backupByBookId(@Param("bookId") String bookId);
}
