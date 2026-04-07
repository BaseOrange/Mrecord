package com.dcz.mrecord.entity;

import com.mybatisflex.annotation.Column;
import com.mybatisflex.annotation.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

import java.util.Date;

/**
 * 用户实体类
 *
 * @author dcz
 * @since 2026/04/07
 */
@Data
@EqualsAndHashCode(callSuper = true)
@Table(value = "SYS_USER")
public class SysUser extends BaseEntity {

    /**
     * 邮箱
     */
    @Column(value = "MR_EMAIL")
    private String email;

    /**
     * 密码
     */
    @Column(value = "MR_PASSWORD")
    private String password;

    /**
     * 昵称
     */
    @Column(value = "MR_NICKNAME")
    private String nickname;

    /**
     * 状态（0-正常，1-停用，2-注销待生效，3-已注销）
     */
    @Column(value = "MR_STATUS")
    private Integer status;

    /**
     * 账号注销申请时间，用于计算15天冷静期
     */
    @Column(value = "MR_CANCEL_TIME")
    private Date cancelTime;

    /**
     * 邮件提醒功能是否启用（0-关闭，1-开启）
     */
    @Column(value = "MR_REMIND_ENABLED")
    private Integer remindEnabled;

    /**
     * 月度提醒日期（取值1-31，无对应日期取月末）
     */
    @Column(value = "MR_REMIND_DAY")
    private Integer remindDay;
}
