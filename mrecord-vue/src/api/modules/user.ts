import {post} from '@/utils/request'
import type {BaseEntity, PageParams, PageResult} from '@/api/types'

// ==================== 类型定义 ====================

/** 用户信息实体 */
export interface SysUser extends BaseEntity {
    /** 邮箱 */
    email?: string
    /** 密码 */
    password?: string
    /** 昵称 */
    nickname?: string
    /** 是否管理员（0-正常用户，1-管理员） */
    admin?: number
    /** 状态（0-正常，1-停用，2-注销待生效，3-已注销） */
    status?: number
    /** 账号注销申请时间 */
    cancelTime?: string
    /** 邮件提醒功能是否启用（0-关闭，1-开启） */
    remindEnabled?: number
    /** 月度提醒日期（1-31） */
    remindDay?: number
}

/** 登录/注册/忘记密码/重置密码/修改信息/管理员重置密码 共用传参 */
export interface UserAuthParams {
    /** 邮箱 */
    email?: string
    /** 密码 */
    password?: string
    /** 昵称 */
    nickname?: string
}

/** 用户列表查询传参 */
export interface ListUsersParams extends PageParams {
    /** 昵称 */
    nickname?: string
    /** 邮箱 */
    email?: string
    /** 状态 */
    status?: number
    /** 是否管理员 */
    isAdmin?: number
}

// ==================== 接口方法 ====================

/** 用户注册 */
export function register(data: UserAuthParams) {
    return post<string>('/user/register', data)
}

/** 用户登录 */
export function login(data: UserAuthParams) {
    return post<string>('/user/login', data)
}

/** 用户退出登录 */
export function logout() {
    return post<void>('/user/logout')
}

/** 忘记密码 */
export function forgotPassword(data: UserAuthParams) {
    return post<void>('/user/forgotPassword', data)
}

/** 重置密码 */
export function resetPassword(data: UserAuthParams) {
    return post<void>('/user/resetPassword', data)
}

/** 查询当前用户信息 */
export function queryMyInfo() {
    return post<SysUser>('/user/queryMyInfo')
}

/** 修改当前用户信息 */
export function updateMyInfo(data: UserAuthParams) {
    return post<SysUser>('/user/updateMyInfo', data)
}

/** 注销当前用户 */
export function canceledMyUser() {
    return post<void>('/user/canceledMyUser')
}

/** 管理员查询所有用户 */
export function listUsers(data?: ListUsersParams) {
    return post<PageResult<SysUser>>('/user/list', data)
}

/** 管理员查询用户信息 */
export function queryUserInfo(userId: string) {
    return post<SysUser>('/user/queryUserInfo', userId)
}

/** 管理员重置密码 */
export function adminResetPassword(data: UserAuthParams) {
    return post<void>('/user/adminResetPassword', data)
}

/** 管理员启用或禁用用户 */
export function enableOrDisableUser(userIdList: string[]) {
    return post<void>('/user/enableOrDisableUser', userIdList)
}

/** 管理员删除用户 */
export function deleteUser(userIdList: string[]) {
    return post<void>('/user/deleteUser', userIdList)
}
