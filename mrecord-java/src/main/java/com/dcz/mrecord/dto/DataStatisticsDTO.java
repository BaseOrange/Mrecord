package com.dcz.mrecord.dto;

import com.dcz.mrecord.entity.FinMonthRecord;
import lombok.Data;

import java.util.List;

/**
 * 数据统计DTO
 */
@Data
public class DataStatisticsDTO<T extends FinMonthRecord> {
    /**
     * 开始年月yyyyMM
     */
    private String startYearMonth;

    /**
     * 结束年月yyyyMM
     */
    private String endYearMonth;

    /**
     * 获取所有账户最新月份统计数据
     */
    private List<T> recordList;
}
