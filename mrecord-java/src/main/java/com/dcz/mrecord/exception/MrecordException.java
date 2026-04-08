package com.dcz.mrecord.exception;

import com.dcz.mrecord.common.ResCode;
import lombok.Getter;

/**
 * 自定义异常
 *
 * @author dcz
 * @since 2026/04/08
 */
@Getter
public class MrecordException extends RuntimeException {
    /**
     * 状态码
     */
    private final String code;

    /**
     * 使用枚举状态码
     *
     * @param resCode 枚举状态码
     */
    public MrecordException(ResCode resCode) {
        super(resCode.getMessage());
        this.code = resCode.getCode();
    }

    /**
     * 自定义消息
     *
     * @param code    状态码
     * @param message 消息
     */
    public MrecordException(String code, String message) {
        super(message);
        this.code = code;
    }
}
