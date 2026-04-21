package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 月度汇总DTO
 *
 * @author dcz
 * @date 2026/04/19
 */
@Data
public class MonthRecordDTO {

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
     * 备注
     */
    private String note;
}
