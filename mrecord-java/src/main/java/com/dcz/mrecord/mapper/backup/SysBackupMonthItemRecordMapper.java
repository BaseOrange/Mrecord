package com.dcz.mrecord.mapper.backup;

import com.dcz.mrecord.entity.backup.SysBackupMonthItemRecord;
import com.mybatisflex.core.BaseMapper;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;

/**
 * 月度财务明细项备份Mapper
 *
 * @author dcz
 * @since 2026/04/07
 */
@Mapper
public interface SysBackupMonthItemRecordMapper extends BaseMapper<SysBackupMonthItemRecord> {

    /**
     * 根据账簿ID备份月度财务明细项数据
     *
     * @param bookId 账簿ID
     */
    void backupByBookId(@Param("bookId") String bookId);
}
