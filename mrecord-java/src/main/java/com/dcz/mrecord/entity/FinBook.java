package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 财务账簿实体类
 * 存储用户创建的年度/分类账簿，实现多账簿独立管理
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "FIN_BOOK")
public class FinBook extends BaseEntity {

    /**
     * 操作用户ID，关联SYS_USER.MR_ID
     */
    @Column(value = "MR_USER_ID")
    private String userId;

    /**
     * 账簿名称，用户自定义
     */
    @Column(value = "MR_BOOK_NAME")
    private String bookName;
}
