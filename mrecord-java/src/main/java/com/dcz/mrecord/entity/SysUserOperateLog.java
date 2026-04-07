package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 用户操作审计日志实体类
 * 记录用户所有关键操作，用于操作追溯、安全审计，不可修改删除
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_USER_OPERATE_LOG")
public class SysUserOperateLog extends BaseEntity {

    /**
     * 操作用户ID，关联SYS_USER.MR_ID
     */
    @Column(value = "MR_USER_ID")
    private String userId;

    /**
     * 操作类型
     * （LOGIN-登录，LOGOUT-登出，UPDATE-数据修改，EXPORT-导出，CANCEL-注销/撤销注销，RESET_PWD-密码重置）
     */
    @Column(value = "MR_OPERATE_TYPE")
    private String operateType;

    /**
     * 操作内容详细描述
     */
    @Column(value = "MR_CONTENT")
    private String content;

    /**
     * 操作IP地址
     */
    @Column(value = "MR_IP")
    private String ip;
}
