package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 账簿导出请求DTO
 *
 * @author dcz
 * @since 2026/04/21
 */
@Data
public class ExportBookDTO {

    /**
     * 账簿ID，不传则导出全部账簿
     */
    private String bookId;

    /**
     * 导出起始年月，格式yyyyMM，不传则从最早数据开始
     */
    private String startYearMonth;

    /**
     * 导出结束年月，格式yyyyMM，不传则到最新数据结束
     */
    private String endYearMonth;
}
