package com.dcz.mrecord.mapper;

import com.dcz.mrecord.entity.SysUserOperateLog;
import com.mybatisflex.core.BaseMapper;
import org.apache.ibatis.annotations.Mapper;

/**
 * 用户操作审计日志Mapper
 *
 * @author dcz
 * @since 2026/04/07
 */
@Mapper
public interface SysUserOperateLogMapper extends BaseMapper<SysUserOperateLog> {
}
