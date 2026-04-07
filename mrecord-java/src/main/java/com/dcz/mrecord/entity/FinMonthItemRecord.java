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
     * 关联月度汇总ID，FIN_MONTH_RECORD.MR_ID
     */
    @Column(value = "MR_RECORD_ID")
    private String recordId;

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
