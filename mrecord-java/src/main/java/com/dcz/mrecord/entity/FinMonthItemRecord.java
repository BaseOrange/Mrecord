package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

import java.math.BigDecimal;

/**
 * 月度财务明细项实体类
 * 存储每月各记账项的具体金额，与月度汇总表一一对应
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "FIN_MONTH_ITEM_RECORD")
public class FinMonthItemRecord extends BaseEntity {

    /**
     * 统计年份
     */
    @Column(value = "MR_YEAR")
    private Integer year;

    /**
     * 统计月份
     */
    @Column(value = "MR_MONTH")
    private Integer month;

    /**
     * 关联账簿 ID，FIN_BOOK.MR_ID
     */
    @Column(value = "MR_BOOK_ID")
    private String bookId;

    /**
     * 关联模板项ID，FIN_TEMPLATE_ITEM.MR_ID
     */
    @Column(value = "MR_TEMPLATE_ITEM_ID")
    private String templateItemId;

    /**
     * 当月该记账项实际金额
     */
    @Column(value = "MR_ITEM_VALUE")
    private BigDecimal itemValue;

}
