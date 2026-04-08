package com.dcz.mrecord.exception;

import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.Result;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

/**
 * 全局异常处理
 *
 * @author dcz
 * @date 2026/04/08
 */
@Slf4j
@RestControllerAdvice
public class GlobalExceptionHandler {
    /**
     * 捕获自定义业务异常
     *
     * @param e 自定义业务异常
     * @return 响应数据
     */
    @ExceptionHandler(MrecordException.class)
    public Result<Void> handleBusinessException(MrecordException e) {
        log.error("业务异常：{}", e.getMessage());
        return Result.fail(e.getCode(), e.getMessage());
    }

    /**
     * 捕获参数校验异常(@Valid)
     *
     * @param e 参数校验异常
     * @return 响应数据
     */
    @ExceptionHandler(MethodArgumentNotValidException.class)
    public Result<Void> handleValidException(MethodArgumentNotValidException e) {
        String msg = e.getBindingResult().getFieldError().getDefaultMessage();
        log.error("参数校验异常：{}", msg);
        return Result.fail(ResCode.PARAM_ERROR.getCode(), msg);
    }

    /**
     * 捕获空指针
     *
     * @param e 空指针
     * @return 响应数据
     */
    @ExceptionHandler(NullPointerException.class)
    public Result<Void> handleNullPointerException(NullPointerException e) {
        log.error("空指针异常", e);
        return Result.fail(ResCode.ERROR.getCode(), "发生数据取值异常，请联系管理员。");
    }

    /**
     * 捕获所有其他异常（兜底）
     *
     * @param e 其他异常
     * @return 响应数据
     */
    @ExceptionHandler(Exception.class)
    public Result<Void> handleException(Exception e) {
        log.error("服务器异常", e);
        return Result.fail(ResCode.ERROR);
    }
}
