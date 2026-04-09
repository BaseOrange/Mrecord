package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.FinMonthItemRecord;
import com.dcz.mrecord.mapper.FinMonthItemRecordMapper;
import com.dcz.mrecord.service.FinMonthItemRecordService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 月度财务明细项服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class FinMonthItemRecordServiceImpl extends ServiceImpl<FinMonthItemRecordMapper, FinMonthItemRecord> implements FinMonthItemRecordService {
}
