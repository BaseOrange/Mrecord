package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.FinTemplateItem;
import com.dcz.mrecord.mapper.FinTemplateItemMapper;
import com.dcz.mrecord.service.FinTemplateItemService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 记账模板明细服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class FinTemplateItemServiceImpl extends ServiceImpl<FinTemplateItemMapper, FinTemplateItem> implements FinTemplateItemService {
}
