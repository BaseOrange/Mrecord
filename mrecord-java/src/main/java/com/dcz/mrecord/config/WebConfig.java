package com.dcz.mrecord.config;

import com.dcz.mrecord.interceptor.CheckAdminInterceptor;
import com.dcz.mrecord.interceptor.LogInterceptor;
import com.dcz.mrecord.interceptor.LoginInterceptor;
import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.PathMatchConfigurer;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@Configuration
@RequiredArgsConstructor
public class WebConfig implements WebMvcConfigurer {
    private final LoginInterceptor loginInterceptor;
    private final CheckAdminInterceptor checkAdminInterceptor;
    private final LogInterceptor logInterceptor;

    @Override
    public void configurePathMatch(PathMatchConfigurer configurer) {
        configurer.addPathPrefix("/api/v2", c -> c.isAnnotationPresent(RestController.class));
    }

    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        registry.addInterceptor(loginInterceptor)
                .addPathPatterns("/api/v2/**")
                .excludePathPatterns(
                        "/api/v2/user/register",
                        "/api/v2/user/login",
                        "/api/v2/user/forgotPassword",
                        "/api/v2/user/resetPassword",
                        "/api/v2/config/initialized",
                        "/api/v2/config/registerEnabled",
                        "/api/v2/config/initAdmin"
                );

        registry.addInterceptor(checkAdminInterceptor)
                .addPathPatterns("/api/v2/**");

        registry.addInterceptor(logInterceptor)
                .addPathPatterns("/api/v2/**")
                .excludePathPatterns(
                        "/api/v2/operateLog/list",
                        "/api/v2/config/initialized",
                        "/api/v2/config/registerEnabled"
                );
    }
}
