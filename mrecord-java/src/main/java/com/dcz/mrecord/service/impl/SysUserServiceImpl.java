package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.mapper.SysUserMapper;
import com.dcz.mrecord.service.SysUserService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 用户服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysUserServiceImpl extends ServiceImpl<SysUserMapper, SysUser> implements SysUserService {
}
