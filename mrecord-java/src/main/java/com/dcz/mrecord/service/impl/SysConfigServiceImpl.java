package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.SysConfig;
import com.dcz.mrecord.mapper.SysConfigMapper;
import com.dcz.mrecord.service.SysConfigService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 配置项服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class SysConfigServiceImpl extends ServiceImpl<SysConfigMapper, SysConfig> implements SysConfigService {
}
