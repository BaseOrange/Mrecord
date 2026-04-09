package com.dcz.mrecord.service.impl;

import com.dcz.mrecord.entity.FinBook;
import com.dcz.mrecord.mapper.FinBookMapper;
import com.dcz.mrecord.service.FinBookService;
import com.mybatisflex.spring.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

/**
 * 财务账簿服务实现
 *
 * @author dcz
 * @since 2026/04/09
 */
@Service
public class FinBookServiceImpl extends ServiceImpl<FinBookMapper, FinBook> implements FinBookService {
}
