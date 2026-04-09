package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.backup.SysBackupUser;
import com.dcz.mrecord.mapper.backup.SysBackupUserMapper;
import com.dcz.mrecord.service.SysBackupUserService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 用户备份服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysBackupUserServiceImpl extends ServiceImpl<SysBackupUserMapper, SysBackupUser> implements SysBackupUserService {
}
