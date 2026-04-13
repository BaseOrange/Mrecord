package com.dcz.mrecord.service;

import com.dcz.mrecord.dto.UserRegisterDTO;
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
    String userRegister(UserRegisterDTO params);
}
