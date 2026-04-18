package com.dcz.mrecord.dto;

import com.dcz.mrecord.entity.FinMonthItemRecord;
import lombok.Data;

import java.util.List;

/**
 * 月度明细DTO
 *
 * @author dcz
 * @date 2026/04/18
 */
@Data
public class MonthItemDTO {

    /**
     * 账簿ID
     */
    private String bookId;

    /**
     * 年份
     */
    private Integer year;

    /**
     * 月份
     */
    private Integer month;

    /**
     * 明细列表
     */
    private List<FinMonthItemRecord> itemList;
}
