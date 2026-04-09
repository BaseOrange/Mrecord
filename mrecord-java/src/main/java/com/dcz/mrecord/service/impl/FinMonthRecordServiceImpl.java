package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.FinMonthRecord;
import com.dcz.mrecord.mapper.FinMonthRecordMapper;
import com.dcz.mrecord.service.FinMonthRecordService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 月度财务汇总服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class FinMonthRecordServiceImpl extends ServiceImpl<FinMonthRecordMapper, FinMonthRecord> implements FinMonthRecordService {
}
