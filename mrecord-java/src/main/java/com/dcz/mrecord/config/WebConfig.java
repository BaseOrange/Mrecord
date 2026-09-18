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
                        // 【激活流程必须免登录】新用户注册后处于「未激活」状态，登录接口会直接拒绝
                        // （UserStatusError），激活页（/activate-account，前端 PUBLIC_PAGES）只能在
                        // 未登录态调用这两个接口完成激活；原先漏配导致激活链接一点就 401，
                        // 用户既无法激活也无法登录，形成注册死路
                        "/api/v2/user/activate",
                        "/api/v2/user/resendActivateEmail",
                        "/api/v2/user/forgotPassword",
                        "/api/v2/user/resetPassword",
                        // 【撤销注销必须免登录】用户注销后处于「注销待生效」状态，登录接口会直接拒绝
                        // （11007），撤销注销入口只能出现在登录页，必须在未登录态调用
                        "/api/v2/user/revokeCancel",
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
