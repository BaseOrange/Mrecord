package com.dcz.mrecord.controller;

import cn.hutool.core.util.DesensitizedUtil;
import cn.hutool.core.util.ObjUtil;
import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.UserDTO;
import com.dcz.mrecord.service.SysUserService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 用户控制器
 *
 * @author dcz
 * @since 2026/04/09
 */
@Slf4j
@RestController
@RequestMapping("/user")
public class SysUserController {

    @Resource
    private SysUserService sysUserService;

    /**
     * 用户注册
     *
     * @param params 注册参数
     * @return 注册结果
     */
    @PostMapping("/register")
    public Result<String> register(UserDTO params) {
        // 数据脱敏后打印日志
        UserDTO clone = ObjUtil.clone(params);
        clone.setPassword(DesensitizedUtil.password(params.getPassword()));
        log.info("用户注册[/user/register]请求传参：{}", params);

        String email = sysUserService.userRegister(params);
        return Result.success(email);
    }

    /**
     * 用户登录
     *
     * @param params 登录参数
     * @return token
     */
    @PostMapping("/login")
    public Result<String> regiloginster(UserDTO params) {
        // 数据脱敏后打印日志
        UserDTO clone = ObjUtil.clone(params);
        clone.setPassword(DesensitizedUtil.password(params.getPassword()));
        log.info("用户登录[/user/login]请求传参：{}", clone);

        String email = sysUserService.login(params);
        return Result.success(email);
    }
}
