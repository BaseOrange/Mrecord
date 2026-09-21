package com.dcz.mrecord.dto;

import lombok.Data;

/**
 * 删除账本模板项DTO
 *
 * @author dcz
 * @since 2026/09/21
 */
@Data
public class DeleteTempItemDTO {

    /**
     * 账本ID
     */
    private String bookId;

    /**
     * 账目模板项ID
     */
    private String templateItemId;
}
