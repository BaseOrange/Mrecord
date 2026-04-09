package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.backup.SysBackupBook;
import com.dcz.mrecord.mapper.backup.SysBackupBookMapper;
import com.dcz.mrecord.service.SysBackupBookService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 财务账簿备份服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysBackupBookServiceImpl extends ServiceImpl<SysBackupBookMapper, SysBackupBook> implements SysBackupBookService {
}
