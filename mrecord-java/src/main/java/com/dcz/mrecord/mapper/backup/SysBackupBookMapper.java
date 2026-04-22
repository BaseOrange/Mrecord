package com.dcz.mrecord.mapper.backup;

import com.dcz.mrecord.entity.backup.SysBackupBook;
import com.mybatisflex.core.BaseMapper;
import org.apache.ibatis.annotations.Mapper;
import org.apache.ibatis.annotations.Param;

/**
 * 财务账簿备份Mapper
 *
 * @author dcz
 * @since 2026/04/07
 */
@Mapper
public interface SysBackupBookMapper extends BaseMapper<SysBackupBook> {

    /**
     * 根据账簿ID备份账簿数据
     *
     * @param bookId 账簿ID
     */
    void backupByBookId(@Param("bookId") String bookId);
}
