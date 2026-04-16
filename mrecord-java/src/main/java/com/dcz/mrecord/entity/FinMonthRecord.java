package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

import java.math.BigDecimal;

/**
 * 月度财务汇总实体类
 * 存储每月财务数据汇总指标，自动计算总资产、总负债、净资产及同比环比
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "FIN_MONTH_RECORD")
public class FinMonthRecord extends BaseEntity {

    /**
     * 所属用户ID，关联SYS_USER.MR_ID
     */
    @Column(value = "MR_USER_ID")
    private String userId;

    /**
     * 所属账簿ID，关联FIN_BOOK.MR_ID
     */
    @Column(value = "MR_BOOK_ID")
    private String bookId;

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
     * 当月总资产，所有资产项之和
     */
    @Column(value = "MR_TOTAL_ASSET")
    private String totalAsset;

    /**
     * 当月总负债，所有负债项之和
     */
    @Column(value = "MR_TOTAL_LIABILITY")
    private BigDecimal totalLiability;

    /**
     * 当月净资产，总资产-总负债
     */
    @Column(value = "MR_NET_ASSET")
    private BigDecimal netAsset;

    /**
     * 环比增长/下跌金额，对比上月
     */
    @Column(value = "MR_MONTH_ON_MONTH")
    private BigDecimal monthOnMonth;

    /**
     * 同比增长/下跌金额，对比去年同月
     */
    @Column(value = "MR_YEAR_ON_YEAR")
    private BigDecimal yearOnYear;

    /**
     * 用户本月汇总备注
     */
    @Column(value = "MR_NOTE")
    private String note;
}
