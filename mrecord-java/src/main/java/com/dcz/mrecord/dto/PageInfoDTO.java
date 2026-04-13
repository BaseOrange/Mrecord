package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 分页信息
 *
 * @author dcz
 * @since 2026/04/13
 */
@Data
public class PageInfoDTO {
    /**
     * 页码
     */
    private Integer pageNum = 1;

    /**
     * 每页数量
     */
    private Integer pageSize = 10;
}
