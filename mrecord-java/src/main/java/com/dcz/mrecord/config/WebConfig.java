package com.dcz.mrecord.config;

import com.dcz.mrecord.interceptor.CheckAdminInterceptor;
import com.dcz.mrecord.interceptor.LogInterceptor;
import com.dcz.mrecord.interceptor.LoginInterceptor;
import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

/**
 * Web配置
 *
 * @author dcz
 * @since 2026/04/10
 */
@Configuration
@RequiredArgsConstructor
public class WebConfig implements WebMvcConfigurer {
    private final LoginInterceptor loginInterceptor;
    private final CheckAdminInterceptor checkAdminInterceptor;
    private final LogInterceptor logInterceptor;

    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        registry.addInterceptor(loginInterceptor)
                .addPathPatterns("/**")
                .excludePathPatterns(
                        // 注册、登录、忘记密码、重置密码放行
                        "/user/register",
                        "/user/login",
                        "/user/forgotPassword",
                        "/user/resetPassword",
                        // 系统初始化状态检查放行
                        "/config/initialized"
                );

        // 管理员权限校验拦截器，必须在登录拦截器之后
        registry.addInterceptor(checkAdminInterceptor)
                .addPathPatterns("/**");

        // 操作日志拦截器
        registry.addInterceptor(logInterceptor)
                .addPathPatterns("/**")
                .excludePathPatterns("/operateLog/list", "/config/initialized");
    }
}
