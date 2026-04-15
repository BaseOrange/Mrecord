package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 查询账簿参数
 *
 * @author dcz
 * @since 2026/04/14
 */
@Data
public class QueryFinBookDTO extends PageInfoDTO {

    /**
     * 账簿名称
     */
    private String name;

    /**
     * 账簿类型
     */
    private String type;

    /**
     * 账簿年份
     */
    private String year;
}
