package com.dcz.mrecord.config;

import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.extern.slf4j.Slf4j;
import org.springframework.core.Ordered;
import org.springframework.core.annotation.Order;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import java.io.IOException;

/**
 * SPA 路由转发过滤器 —— 对所有非 API、非静态资源的请求，统一返回 /index.html，
 * 由前端 Vue Router 负责客户端路由解析。
 *
 * 使用 HIGHEST_PRECEDENCE 确保此 Filter 在最前面执行，避免其他 Filter 或
 * DispatcherServlet 提前处理（如抛出 NoResourceFoundException 导致白屏）。
 */
@Slf4j
@Component
@Order(Ordered.HIGHEST_PRECEDENCE)
public class SpaForwardFilter extends OncePerRequestFilter {

    @Override
    protected void doFilterInternal(HttpServletRequest request, HttpServletResponse response,
                                    FilterChain chain) throws ServletException, IOException {
        String path = request.getRequestURI();

        log.debug("SPA Filter: path={}", path);

        // API 请求：放行
        if (path.startsWith("/api/")) {
            chain.doFilter(request, response);
            return;
        }

        // 静态资源（含文件后缀）：放行
        if (path.contains(".")) {
            if (path.startsWith("/.well-known/")) {
                response.sendError(HttpServletResponse.SC_NOT_FOUND);
                return;
            }
            chain.doFilter(request, response);
            return;
        }

        // SPA 前端路由：转发到 index.html
        log.debug("SPA Filter: forwarding {} -> /index.html", path);
        response.setHeader("Cache-Control", "no-cache, no-store, must-revalidate");
        response.setHeader("Pragma", "no-cache");
        response.setHeader("Expires", "0");
        request.getRequestDispatcher("/index.html").forward(request, response);
    }
}
