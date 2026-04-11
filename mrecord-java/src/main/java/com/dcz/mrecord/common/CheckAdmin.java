package com.dcz.mrecord.common;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

/**
 * 管理员权限校验注解
 * 加在 Controller 方法上，会自动校验当前用户是否为管理员
 *
 * @author dcz
 * @since 2026/04/11
 */
@Target({ElementType.METHOD})
@Retention(RetentionPolicy.RUNTIME)
public @interface CheckAdmin {
}
