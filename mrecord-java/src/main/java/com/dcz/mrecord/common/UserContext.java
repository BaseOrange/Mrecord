package com.dcz.mrecord.common;

/**
 * 用户上下文
 *
 * @author dcz
 * @since 2026/04/10
 */
public class UserContext {
    private static final ThreadLocal<String> USER_ID = new ThreadLocal<>();

    public static String getUserId() {
        return USER_ID.get();
    }

    public static void setUserId(String userId) {
        USER_ID.set(userId);
    }

    public static void clear() {
        USER_ID.remove();
    }
}
