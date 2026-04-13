package com.dcz.mrecord.config;

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

    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        registry.addInterceptor(loginInterceptor)
                .addPathPatterns("/api/v2/**")
                .excludePathPatterns(
                        // 注册、登录、忘记密码、重置密码放行
                        "/api/v2/user/register",
                        "/api/v2/user/login",
                        "/api/v2/user/forgotPassword",
                        "/api/v2/user/resetPassword"
                );
    }
}
