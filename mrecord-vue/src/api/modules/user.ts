import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface RegisterParams {
    username: string
    password: string
    nickname?: string
}

export interface LoginParams {
    username: string
    password: string
}

export interface LoginResult {
    token: string
    userInfo: UserInfo
}

export interface UserInfo {
    id: number | string
    username: string
    nickname?: string
    avatar?: string
    email?: string
    phone?: string
    status?: number
    role?: string
    createTime?: string
}

export interface ForgotPasswordParams {
    username: string
    email?: string
    phone?: string
}

export interface ResetPasswordParams {
    token: string
    newPassword: string
}

export interface UpdateMyInfoParams {
    nickname?: string
    avatar?: string
    email?: string
    phone?: string
}

export interface AdminResetPasswordParams {
    userId: number | string
    newPassword: string
}

export interface EnableOrDisableUserParams {
    userId: number | string
    enabled: boolean
}

export interface DeleteUserParams {
    userId: number | string
}

export interface ListUsersParams {
    page?: number
    pageSize?: number
    keyword?: string
    status?: number
}

export interface ListUsersResult {
    list: UserInfo[]
    total: number
    page: number
    pageSize: number
}

export interface QueryUserInfoParams {
    userId: number | string
}

// ==================== 接口方法 ====================

/** 用户注册 */
export function register(data: RegisterParams) {
    return post<LoginResult>('/user/register', data)
}

/** 用户登录 */
export function login(data: LoginParams) {
    return post<LoginResult>('/user/login', data)
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
    return post<UserInfo>('/user/queryMyInfo')
}

/** 修改当前用户信息 */
export function updateMyInfo(data: UpdateMyInfoParams) {
    return post<UserInfo>('/user/updateMyInfo', data)
}

/** 注销当前用户 */
export function canceledMyUser() {
    return post<void>('/user/canceledMyUser')
}

/** 管理员查询所有用户 */
export function listUsers(data?: ListUsersParams) {
    return post<ListUsersResult>('/user/list', data)
}

/** 管理员查询用户信息 */
export function queryUserInfo(data: QueryUserInfoParams) {
    return post<UserInfo>('/user/queryUserInfo', data)
}

/** 管理员重置密码 */
export function adminResetPassword(data: AdminResetPasswordParams) {
    return post<void>('/user/adminResetPassword', data)
}

/** 管理员启用或禁用用户 */
export function enableOrDisableUser(data: EnableOrDisableUserParams) {
    return post<void>('/user/enableOrDisableUser', data)
}

/** 管理员删除用户 */
export function deleteUser(data: DeleteUserParams) {
    return post<void>('/user/deleteUser', data)
}
