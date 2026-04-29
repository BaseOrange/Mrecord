package com.dcz.mrecord.dto;

import com.dcz.mrecord.entity.FinMonthRecord;
import lombok.Data;

/**
 * 财务记录DTO
 *
 * @author dcz
 * @since 2026/4/29
 */
@Data
public class FinBookRecordDTO extends FinMonthRecord {
    /**
     * 账簿ID
     */
    private String bookId;

    /**
     * 账簿名称
     */
    private String bookName;
}
