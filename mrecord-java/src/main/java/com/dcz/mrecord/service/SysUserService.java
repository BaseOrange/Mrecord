package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.ChangePasswordDTO;
import com.dcz.mrecord.dto.InitAdminDTO;
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
     * 初始化管理员账户
     *
     * @param params 管理员信息
     * @return 管理员用户ID
     */
    String initAdmin(InitAdminDTO params);

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
     * 撤销注销（冷静期内恢复账户）
     *
     * <p>注销冷静期内用户无法登录，因此本接口免登录鉴权，改为凭邮箱+密码确认身份。
     * 仅「注销待生效」状态可撤销，恢复为正常状态并清空注销申请时间。</p>
     *
     * @param params 邮箱 + 密码
     */
    void revokeCancel(UserDTO params);

    /**
     * 清理已过冷静期的待注销用户
     *
     * <p>备份并删除用户名下的全部账簿数据后，物理删除用户本体。对应 Rust 端
     * {@code service::cancel_cleanup_task::cleanup_user}，由注销清理定时任务调用。</p>
     *
     * @param sysUser 待清理用户
     */
    void cleanupCanceledUser(SysUser sysUser);

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
    Page<SysUser> queryList(QueryUserDTO params);

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

    /**
     * 修改密码
     *
     * @param params 修改密码参数
     */
    void changePassword(ChangePasswordDTO params);

    /**
     * 激活账户
     *
     * @param token 激活令牌
     */
    void activateAccount(String token);

    /**
     * 重新发送激活邮件
     *
     * @param email 用户邮箱
     */
    void resendActivateEmail(String email);

}
