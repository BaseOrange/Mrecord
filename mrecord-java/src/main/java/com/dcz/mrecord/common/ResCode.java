package com.dcz.mrecord.common;

import lombok.Getter;

/**
 * 响应码枚举
 *
 * @author dcz
 * @since 2026/04/08
 */
@Getter
public enum ResCode {
    /**
     * 成功
     */
    SUCCESS("00000", "请求成功"),

    /**
     * 客户端错误
     */
    PARAM_ERROR("10001", "请求参数错误"),
    UNAUTHORIZED("10002", "用户已存在，注册失败"),
    LOGIN_INFO_ERROR("10003", "账号或密码错误"),
    LOGIN_EXPIR("10004", "登录过期"),
    NO_PERMISSION("10005", "无操作权限"),
    DATA_NOT_EXIST("10006", "数据不存在"),
    DATA_DUPLICATION("10007", "数据重复，操作失败"),
    ASYNC_PROCESS("10008", "异步任务处理中"),
    OPERATION_FAIL("10009", "操作执行失败"),

    /**
     * 服务端错误
     */
    ERROR("50000", "服务器内部错误"),
    BUSINESS_ERROR("50001", "业务异常");

    private final String code;
    private final String message;

    ResCode(String code, String message) {
        this.code = code;
        this.message = message;
    }
}
