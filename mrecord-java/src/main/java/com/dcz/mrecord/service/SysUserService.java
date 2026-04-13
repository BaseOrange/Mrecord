package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.UserDTO;
import com.dcz.mrecord.entity.SysUser;
import com.mybatisflex.core.service.IService;

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
}
