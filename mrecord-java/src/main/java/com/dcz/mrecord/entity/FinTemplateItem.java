package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 记账模板明细实体类
 * 存储每个账簿的自定义资产/负债记账项，支持图标、排序设置
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "FIN_TEMPLATE_ITEM")
public class FinTemplateItem extends BaseEntity {

    /**
     * 所属账簿ID，关联FIN_BOOK.MR_ID
     */
    @Column(value = "MR_BOOK_ID")
    private String bookId;

    /**
     * 记账项名称，如招行储蓄卡、花呗
     */
    @Column(value = "MR_ITEM_NAME")
    private String itemName;

    /**
     * 账簿类型：(-1:负债，0:不统计仅记录，1:资产)
     */
    @Column(value = "MR_ITEM_TYPE")
    private Integer itemType;

    /**
     * 图标标识，对应系统内置图标库
     */
    @Column(value = "MR_ICON")
    private String icon;

    /**
     * 展示排序号，数值越小越靠前
     */
    @Column(value = "MR_SORT")
    private String sort;
}
