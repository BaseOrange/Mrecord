package com.dcz.mrecord.dto;

import com.dcz.mrecord.entity.FinTemplateItem;
import lombok.Data;

import java.util.List;

/**
 * 创建账本模板项DTO
 *
 * @author dcz
 * @since 2023/09/15
 */
@Data
public class FinTempItemDTO {

    /**
     * 账本ID
     */
    private String bookId;

    /**
     * 旧账簿ID
     */
    private String oldBookId;

    /**
     * 账本模板项
     */
    private List<FinTemplateItem> itemList;
}
