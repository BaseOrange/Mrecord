package com.dcz.mrecord.interceptor;

import com.dcz.mrecord.common.CheckAdmin;
import com.dcz.mrecord.common.ResCode;
import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.common.UserContext;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.mapper.SysUserMapper;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.RequiredArgsConstructor;
import org.jspecify.annotations.NonNull;
import org.springframework.stereotype.Component;
import org.springframework.web.method.HandlerMethod;
import org.springframework.web.servlet.HandlerInterceptor;
import tools.jackson.databind.ObjectMapper;

/**
 * 管理员拦截器
 *
 * @author dcz
 * @since 2026/04/11
 */
@Component
@RequiredArgsConstructor
public class CheckAdminInterceptor implements HandlerInterceptor {
    @Resource
    private final SysUserMapper userMapper;
    @Resource
    private final ObjectMapper objectMapper;

    @Override
    public boolean preHandle(@NonNull HttpServletRequest request, @NonNull HttpServletResponse response, @NonNull Object handler) throws Exception {

        // 只处理 Controller 方法
        if (!(handler instanceof HandlerMethod handlerMethod)) {
            return true;
        }

        // 判断是否加了 @CheckAdmin
        CheckAdmin annotation = handlerMethod.getMethodAnnotation(CheckAdmin.class);
        if (annotation == null) {
            annotation = handlerMethod.getBeanType().getAnnotation(CheckAdmin.class);
        }
        if (annotation == null) {
            return true;
        }

        // 拿到当前登录用户ID
        String userId = UserContext.getUserId();
        SysUser user = userMapper.selectOneById(userId);

        if (user == null || user.getAdmin() == null || user.getAdmin() != 1) {
            response.setContentType("application/json;charset=utf-8");
            response.getWriter().write(objectMapper.writeValueAsString(Result.fail(ResCode.NO_PERMISSION)));
            return false;
        }

        return true;
    }
}
