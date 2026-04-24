package com.dcz.mrecord.controller;

import cn.hutool.core.util.DesensitizedUtil;
import cn.hutool.core.util.ObjUtil;
import com.dcz.mrecord.common.CheckAdmin;
import com.dcz.mrecord.common.Result;
import com.dcz.mrecord.dto.QueryUserDTO;
import com.dcz.mrecord.dto.UserDTO;
import com.dcz.mrecord.entity.SysUser;
import com.dcz.mrecord.service.SysUserService;
import com.mybatisflex.core.paginate.Page;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Set;

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
    public Result<String> login(UserDTO params) {
        // 数据脱敏后打印日志
        UserDTO clone = ObjUtil.clone(params);
        clone.setPassword(DesensitizedUtil.password(params.getPassword()));
        log.info("用户登录[/user/login]请求传参：{}", clone);

        String token = sysUserService.login(params);
        return Result.success(token);
    }

    /**
     * 用户退出登录
     *
     * @return 退出登录结果
     */
    @PostMapping("/logout")
    public Result<String> logout() {
        // 无状态 JWT，后端无需任何处理 前端只需要删除本地 token 就等于退出
        return Result.success();
    }

    /**
     * 忘记密码
     *
     * @return 请求响应
     */
    @PostMapping("/forgotPassword")
    public Result<String> forgotPassword(UserDTO params) throws Exception {
        // 数据脱敏后打印日志
        UserDTO clone = ObjUtil.clone(params);
        clone.setPassword(DesensitizedUtil.password(params.getPassword()));
        log.info("忘记密码[/user/forgotPassword]请求传参：{}", clone);

        sysUserService.forgotPassword(params);
        return Result.success();
    }

    /**
     * 重置密码
     *
     * @return 请求响应
     */
    @PostMapping("/resetPassword")
    public Result<String> resetPassword(UserDTO params) {
        // 数据脱敏后打印日志
        UserDTO clone = ObjUtil.clone(params);
        clone.setPassword(DesensitizedUtil.password(params.getPassword()));
        log.info("重置密码[/user/resetPassword]请求传参：{}", clone);

        sysUserService.resetPassword(params);
        return Result.success();
    }

    /**
     * 查询当前用户信息
     *
     * @return 用户信息
     */
    @PostMapping("/queryMyInfo")
    public Result<SysUser> queryMyInfo() {
        log.info("查询当前用户信息[/user/queryMyInfo]");
        SysUser sysUser = sysUserService.queryMyUserInfo();
        return Result.success(sysUser);
    }

    /**
     * 修改当前用户信息
     *
     * @return 修改结果
     */
    @PostMapping("/updateMyInfo")
    public Result<SysUser> updateMyInfo(UserDTO params) {
        log.info("修改当前用户信息[/user/updateMyInfo]请求传参：{}", params);
        SysUser sysUser = sysUserService.updateMyUserInfo(params);
        return Result.success(sysUser);
    }

    /**
     * 注销当前用户
     *
     * @return 注销结果
     */
    @PostMapping("/canceledMyUser")
    public Result<String> canceledMyUser() {
        log.info("注销当前用户[/user/canceledMyUser]");
        sysUserService.canceledMyUser();
        return Result.success();
    }

    /**
     * 管理员查询所有用户
     *
     * @return 所有用户分页集合
     */
    @CheckAdmin
    @PostMapping("/list")
    public Result<Page<SysUser>> queryAll(QueryUserDTO params) {
        log.info("管理员查询所有用户[/user/list]请求传参：{}", params);
        return Result.success(sysUserService.queryList(params));
    }

    /**
     * 管理员查询用户信息
     *
     * @return 用户信息
     */
    @CheckAdmin
    @PostMapping("/queryUserInfo")
    public Result<SysUser> queryUserInfo(String userId) {
        log.info("管理员查询用户信息[/user/queryUserInfo]请求传参：{}", userId);
        return Result.success(sysUserService.queryUserInfo(userId));
    }

    /**
     * 管理员重置密码
     *
     * @return 重置密码结果
     */
    @CheckAdmin
    @PostMapping("/adminResetPassword")
    public Result<String> adminResetPassword(UserDTO params) {
        // 数据脱敏后打印日志
        UserDTO clone = ObjUtil.clone(params);
        clone.setPassword(DesensitizedUtil.password(params.getPassword()));
        log.info("管理员重置密码[/user/adminResetPassword]请求传参：{}", clone);
        sysUserService.adminResetPassword(params);
        return Result.success();
    }

    /**
     * 管理员启用或禁用用户
     *
     * @return 启用或禁用结果
     */
    @CheckAdmin
    @PostMapping("/enableOrDisableUser")
    public Result<String> enableOrDisableUser(Set<String> userIdList) {
        log.info("管理员启用或禁用用户[/user/enableOrDisableUser]请求传参：{}", userIdList);
        sysUserService.enableOrDisableUser(userIdList);
        return Result.success();
    }

    /**
     * 管理员删除用户
     *
     * @return 删除结果
     */
    @CheckAdmin
    @PostMapping("/deleteUser")
    public Result<String> deleteUser(Set<String> userIdList) {
        log.info("管理员删除用户[/user/deleteUser]请求传参：{}", userIdList);
        sysUserService.deleteUser(userIdList);
        return Result.success();
    }
}
