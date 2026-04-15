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

    // 用户错误
    UNAUTHORIZED("11001", "用户邮箱已存在，注册失败"),
    LOGIN_INFO_ERROR("11002", "账号或密码错误"),
    LOGIN_EXPIR("11003", "登录过期"),
    NO_PERMISSION("11004", "无操作权限"),
    USER_STATUS_ERROR("11005", "用户状态异常"),

    // 数据错误
    DATA_NOT_EXIST("12001", "数据不存在"),
    DATA_DUPLICATION("12002", "数据重复，操作失败"),

    // 异步处理错误
    ASYNC_PROCESS("13008", "异步任务处理中"),
    OPERATION_FAIL("13009", "操作执行失败"),

    // 账簿错误
    FIN_BOOK_NOT_FOUND("14101", "账簿不存在"),
    FIN_BOOK_TYPE_UPDATE("14102", "账簿类型禁止修改，如需修改请删除后重新创建，或创建新账簿"),
    FIN_BOOK_YEAR_UPDATE("14103", "账簿年份禁止修改，如需修改请删除后重新创建，或创建新账簿"),

    // 账目错误
    FIN_ITEM_NOT_FOUND("14201", "账目不存在"),


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
