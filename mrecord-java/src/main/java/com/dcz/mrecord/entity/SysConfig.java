package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

import java.util.Date;

/**
 * 配置项实体类
 * 存储系统全局配置参数，如邮件SMTP配置、注册开关等，仅管理员可修改
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_CONFIG")
public class SysConfig extends BaseEntity {

    /**
     * 配置项key（代码读取唯一标识）
     */
    @Column(value = "MR_CONFIG_KEY")
    private String key;

    /**
     * 配置项value
     */
    @Column(value = "MR_CONFIG_VALUE")
    private String value;

    /**
     * 配置项描述
     */
    @Column(value = "MR_REMARK")
    private String remark;
}
