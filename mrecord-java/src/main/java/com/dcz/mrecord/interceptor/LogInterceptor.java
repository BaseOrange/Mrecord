package com.dcz.mrecord.interceptor;


import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.entity.SysUserOperateLog;
import com.dcz.mrecord.service.SysUserOperateLogService;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.springframework.web.method.HandlerMethod;
import org.springframework.web.servlet.HandlerInterceptor;

import java.io.BufferedReader;

/**
 * 日志拦截器
 *
 * @author dcz
 * @date 2026/4/17
 */
@Slf4j
@Component
@RequiredArgsConstructor
public class LogInterceptor implements HandlerInterceptor {

    @Resource
    private SysUserOperateLogService logService;

    /**
     * 请求进入Controller【之前】执行
     *
     * @param request  current HTTP request
     * @param response current HTTP response
     * @param handler  chosen handler to execute, for type and/or instance evaluation
     * @return
     */
    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) {
        // 记录请求开始时间
        long startTime = System.currentTimeMillis();
        request.setAttribute("requestStartTime", startTime);

        SysUserOperateLog reqLog = new SysUserOperateLog();
        reqLog.setUserId(UserContext.getUserId());
        reqLog.setIp(getClientIp(request));
        reqLog.setOperateType(request.getRequestURI());

        try {
            BufferedReader reader = request.getReader();
            StringBuilder builder = new StringBuilder();
            String line = reader.readLine();
            while (line != null) {
                builder.append(line);
                line = reader.readLine();
            }
            reader.close();
            String reqBody = builder.toString();
            reqLog.setContent(reqBody);
        } catch (Exception e) {
            log.error("日志记录异常", e);
        }

        logService.saveLog(reqLog);
        return true;
    }

    /**
     * 请求处理【完成后】执行
     *
     * @param request  current HTTP request
     * @param response current HTTP response
     * @param handler  the handler (or {@link HandlerMethod}) that started asynchronous
     *                 execution, for type and/or instance examination
     * @param ex       any exception thrown on handler execution, if any; this does not
     *                 include exceptions that have been handled through an exception resolver
     */
    @Override
    public void afterCompletion(HttpServletRequest request, HttpServletResponse response, Object handler, Exception ex) {
        long startTime = (long) request.getAttribute("requestStartTime");
        long costTime = System.currentTimeMillis() - startTime;
        // 请求结束日志
        log.info("请求耗时: {} ms", costTime);
    }

    /**
     * 获取真实客户端IP
     *
     * @param request 请求对象
     * @return 客户端IP
     */
    private String getClientIp(HttpServletRequest request) {
        String ip = request.getHeader("X-Forwarded-For");
        if (ip == null || ip.isEmpty() || "unknown".equalsIgnoreCase(ip)) {
            ip = request.getHeader("Proxy-Client-IP");
        }
        if (ip == null || ip.isEmpty() || "unknown".equalsIgnoreCase(ip)) {
            ip = request.getRemoteAddr();
        }
        return ip;
    }
}
