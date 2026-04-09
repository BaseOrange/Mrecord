package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.SysUserOperateLog;
import com.dcz.mrecord.mapper.SysUserOperateLogMapper;
import com.dcz.mrecord.service.SysUserOperateLogService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 用户操作审计日志服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysUserOperateLogServiceImpl extends ServiceImpl<SysUserOperateLogMapper, SysUserOperateLog> implements SysUserOperateLogService {
}
