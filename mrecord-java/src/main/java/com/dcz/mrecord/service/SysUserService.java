package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.QueryUserDTO;
import com.dcz.mrecord.dto.UserDTO;
import com.dcz.mrecord.entity.SysUser;
import com.mybatisflex.core.paginate.Page;
import com.mybatisflex.core.service.IService;

import java.util.Set;

/**
 * 用户服务
 *
 * @author dcz
 * @since 2026/04/09
 */
public interface SysUserService extends IService<SysUser> {

    /**
     * 用户注册
     *
     * @param params 注册参数
     * @return 注册结果
     */
    String userRegister(UserDTO params);

    /**
     * 用户登录
     *
     * @param params 登录参数
     * @return token值
     */
    String login(UserDTO params);

    /**
     * 忘记密码
     *
     * @param params 忘记密码参数
     */
    void forgotPassword(UserDTO params) throws Exception;

    /**
     * 重置密码
     *
     * @param params 重置密码参数
     */
    void resetPassword(UserDTO params);

    /**
     * 获取当前用户信息
     *
     * @return 用户信息
     */
    SysUser queryMyUserInfo();

    /**
     * 更新当前用户信息
     *
     * @param params 用户信息
     * @return 用户信息
     */
    SysUser updateMyUserInfo(UserDTO params);

    /**
     * 注销我的账户
     */
    void canceledMyUser();

    /**
     * 获取用户信息
     *
     * @param userId 用户ID
     * @return 用户信息
     */
    SysUser queryUserInfo(String userId);

    /**
     * 查询所有用户
     *
     * @return 用户列表
     */
    Page<SysUser> queryAll(QueryUserDTO params);

    /**
     * 管理员重置密码
     *
     * @param params 重置密码参数
     */
    void adminResetPassword(UserDTO params);

    /**
     * 删除用户
     *
     * @param userIdList 用户ID集合
     */
    void deleteUser(Set<String> userIdList);

    /**
     * 启用或禁用用户
     *
     * @param userIdList 用户ID集合
     */
    void enableOrDisableUser(Set<String> userIdList);

}
