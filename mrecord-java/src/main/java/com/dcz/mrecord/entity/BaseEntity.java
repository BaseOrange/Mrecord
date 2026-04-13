package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Id;
import lombok.Data;

import java.util.Date;

/**
 * 基础实体类
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
public class BaseEntity {

    /**
     * 主键
     */
    @Id
    @Column(value = "MR_ID")
    private String id;

    /**
     * 创建人
     */
    @Column(value = "MR_CREATE_BY")
    private String createBy;

    /**
     * 创建时间
     */
    @Column(value = "MR_CREATE_TIME", onInsertValue = "now()")
    private Date createTime;

    /**
     * 更新人
     */
    @Column(value = "MR_UPDATE_BY")
    private String updateBy;

    /**
     * 更新时间
     */
    @Column(value = "MR_UPDATE_TIME", onUpdateValue = "now()", onInsertValue = "now()")
    private Date updateTime;

    /**
     * 逻辑删除标识（0-正常，1-已删除）
     */
    @Column(value = "MR_IS_DELETED", isLogicDelete = true)
    private Integer isDeleted;
}
