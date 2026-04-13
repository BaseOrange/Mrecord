package com.dcz.mrecord.constant;

/**
 * 用户状态常量
 *
 * @author dcz
 * @since 2026/04/13
 */
public interface UserStatusConst {

    /**
     * 正常
     */
    public static final Integer NORMAL = 0;

    /**
     * 停用
     */
    public static final Integer DISABLED = 1;

    /**
     * 注销待生效
     */
    public static final Integer CANCELED_WAIT = 2;

    /**
     * 已注销
     */
    public static final Integer CANCELED = 3;
}
