package com.dcz.mrecord.common;

import cn.hutool.core.util.StrUtil;
import com.dcz.mrecord.exception.MrecordException;

/**
 * 用户上下文
 *
 * @author dcz
 * @since 2026/04/10
 */
public class UserContext {
    private static final ThreadLocal<String> USER_ID = new ThreadLocal<>();

    public static String getUserId() {
        String userId = USER_ID.get();
        if (StrUtil.isBlankIfStr(userId)) {
            throw new MrecordException(ResCode.DATA_NOT_EXIST);
        }
        return userId;
    }

    public static void setUserId(String userId) {
        USER_ID.set(userId);
    }

    public static void clear() {
        USER_ID.remove();
    }
}
