package com.dcz.mrecord.dto;

import com.dcz.mrecord.entity.FinMonthRecord;
import lombok.Data;

import java.util.List;
import java.util.Map;

/**
 * 数据统计DTO
 */
@Data
public class DataStatisticsDTO {
    /**
     * 开始年月yyyyMM
     */
    private String startYearMonth;

    /**
     * 结束年月yyyyMM
     */
    private String endYearMonth;

    /**
     * 数据Map Key:账簿名称 value：近12个月的汇总数据集合
     */
    Map<String, List<FinMonthRecord>> recordMap;
}
