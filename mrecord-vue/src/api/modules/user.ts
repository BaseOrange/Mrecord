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

/** 登录传参 */
export interface LoginParams {
    email: string
    password: string
}

/** 注册传参 */
export interface RegisterParams {
    email: string
    password: string
    nickname: string
}

/** 忘记密码传参 */
export interface ForgotPasswordParams {
    email: string
}

/** 重置密码传参 */
export interface ResetPasswordParams {
    password: string
    rePasswordToken: string
}

/** 撤销注销传参 */
export interface RevokeCancelParams {
    email: string
    password: string
}

/** 管理员重置密码传参 */
export interface AdminResetPasswordParams {
    email: string
    password: string
}

/** 修改当前用户信息传参 */
export interface UpdateMyInfoParams {
    /** 昵称 */
    nickname?: string
    /** 邮件提醒功能是否启用（0-关闭，1-开启） */
    remindEnabled?: number
    /** 月度提醒日期（1-31） */
    remindDay?: number
}

/** 修改密码传参 */
export interface ChangePasswordParams {
    /** 旧密码 */
    oldPassword: string
    /** 新密码 */
    newPassword: string
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
export function register(data: RegisterParams) {
    return post<string>('/user/register', data)
}

/** 账户激活 */
export function activateAccount(activateToken: string) {
    return post<void>('/user/activate', {activateToken})
}

/** 重新发送激活邮件 */
export function resendActivateEmail(email: string) {
    return post<void>('/user/resendActivateEmail', {email})
}

/** 用户登录 */
export function login(data: LoginParams) {
    return post<string>('/user/login', data)
}

/** 用户退出登录 */
export function logout() {
    return post<void>('/user/logout')
}

/** 忘记密码 */
export function forgotPassword(data: ForgotPasswordParams) {
    return post<void>('/user/forgotPassword', data)
}

/** 重置密码 */
export function resetPassword(data: ResetPasswordParams) {
    return post<void>('/user/resetPassword', data)
}

/** 查询当前用户信息 */
export function queryMyInfo() {
    return post<SysUser>('/user/queryMyInfo')
}

/** 修改当前用户信息 */
export function updateMyInfo(data: UpdateMyInfoParams) {
    return post<SysUser>('/user/updateMyInfo', data)
}

/** 注销当前用户 */
export function canceledMyUser() {
    return post<void>('/user/canceledMyUser')
}

/** 撤销注销（冷静期内恢复账户，免登录接口） */
export function revokeCancel(data: RevokeCancelParams) {
    return post<void>('/user/revokeCancel', data)
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
export function adminResetPassword(data: AdminResetPasswordParams) {
    return post<void>('/user/adminResetPassword', data)
}

/** 修改密码 */
export function changePassword(data: ChangePasswordParams) {
    return post<void>('/user/changePassword', data)
}

/** 管理员启用或禁用用户 */
export function enableOrDisableUser(userIdList: string[]) {
    return post<void>('/user/enableOrDisableUser', userIdList)
}

/** 管理员删除用户 */
export function deleteUser(userIdList: string[]) {
    return post<void>('/user/deleteUser', userIdList)
}
