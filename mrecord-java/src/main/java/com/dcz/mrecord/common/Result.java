package com.dcz.mrecord.common;

import lombok.Data;

/**
 * 响应体
 *
 * @author dcz
 * @since 2026/04/08
 */
@Data
public class Result<T> {
    /**
     * 响应码
     */
    private String code;

    /**
     * 响应消息
     */
    private String message;

    /**
     * 响应数据
     */
    private T data;

    /**
     * 私有化构造，禁止外部new
     */
    private Result() {
    }

    /**
     * 成功返回
     *
     * @param <T> 响应的数据类型
     * @return 响应数据
     */
    public static <T> Result<T> success() {
        return success(null);
    }

    /**
     * 成功返回：自定义消息
     *
     * @param data 响应数据
     * @param <T>  响应的数据类型
     * @return 响应数据
     */
    public static <T> Result<T> success(T data) {
        Result<T> result = new Result<>();
        result.setCode(ResCode.SUCCESS.getCode());
        result.setMessage(ResCode.SUCCESS.getMessage());
        result.setData(data);
        return result;
    }

    /**
     * 失败返回
     *
     * @param ResCode 响应码枚举
     * @param <T>     响应的数据类型
     * @return 响应数据
     */
    public static <T> Result<T> fail(ResCode ResCode) {
        Result<T> result = new Result<>();
        result.setCode(ResCode.getCode());
        result.setMessage(ResCode.getMessage());
        return result;
    }

    /**
     * 失败返回：自定义消息
     *
     * @param code    响应码
     * @param message 响应消息
     * @param <T>     响应的数据类型
     * @return 响应数据
     */
    public static <T> Result<T> fail(String code, String message) {
        Result<T> result = new Result<>();
        result.setCode(code);
        result.setMessage(message);
        return result;
    }
}
