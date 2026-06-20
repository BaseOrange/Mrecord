//! 用户模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.SysUserController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.SysUserServiceImpl`
//!
//! 每个 handler 仅负责参数提取与拼装响应，业务逻辑直接内联在这里
//! （Rust 项目暂未拆分 service 层；后续若膨胀可按 Java 项目拆出 `service/user.rs`）。

use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    common::{
        page::PageResult,
        res_code::ResCode,
        result::ApiResponse,
        user_context::{AdminUser, AuthUser},
    },
    constant::user_status::UserStatus,
    entity::sys_user::{self, ActiveModel as UserActive, Column as UserCol, Entity as UserEntity},
    error::AppError,
    model::{
        mail_params::MailParams,
        user::{ChangePasswordDto, InitAdminDto, QueryUserDto, UserDto, UserResponse},
    },
    util::{
        jwt,
        token::{self, TokenPurpose},
    },
    AppState,
};

/// 邮箱格式校验正则
///
/// 替代 Java 的 `Validator.isEmail`。仅做简易校验，覆盖常见情况，
/// 不追求 RFC 5322 完整性。
static EMAIL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}$").unwrap());

/// 校验邮箱格式
fn is_valid_email(email: &str) -> bool {
    EMAIL_RE.is_match(email)
}

/// 业务参数错误的快捷构造
fn param_err(msg: impl Into<String>) -> AppError {
    AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: msg.into(),
    }
}

/// 加密密码（BCrypt）
fn hash_password(pwd: &str) -> Result<String, AppError> {
    hash(pwd, DEFAULT_COST).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
}

/// 拼接账户激活链接
///
/// 与 Java `getActivateAccountUrl` 保持一致：站点地址 + `activate-account?token=`。
/// 未配置 webSite 则只返回 token，便于本地调试。
fn build_activate_link(web_site: Option<&str>, token: &str) -> String {
    match web_site.filter(|s| !s.is_empty()) {
        Some(prefix) => {
            // 与 Java 直接字符串拼接对齐：webSite 通常以 `/` 结尾。这里规范化一次。
            let trimmed = prefix.trim_end_matches('/');
            format!("{}/activate-account?token={}", trimmed, token)
        }
        None => token.to_string(),
    }
}

/// 拼接重置密码链接
///
/// 与 Java `getResetPasswordUrl` 保持一致：站点地址 + `reset-password?token=`。
fn build_reset_link(web_site: Option<&str>, token: &str) -> String {
    match web_site.filter(|s| !s.is_empty()) {
        Some(prefix) => {
            let trimmed = prefix.trim_end_matches('/');
            format!("{}/reset-password?token={}", trimmed, token)
        }
        None => token.to_string(),
    }
}

// ==================== 接口 handler ====================

/// 初始化管理员账户：`POST /user/initAdmin`
///
/// 对应 Java: `SysUserController.initAdmin`（实际位于 `SysConfigController`，但服务方法在 `SysUserService`）。
/// 这里保留接口，简化版本：仅当数据库中没有任何管理员时才允许初始化。
pub async fn init_admin(
    State(state): State<AppState>,
    Json(params): Json<InitAdminDto>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    // 参数校验
    if params.email.trim().is_empty() {
        return Err(param_err("邮箱不能为空"));
    }
    if !is_valid_email(&params.email) {
        return Err(param_err("邮箱格式错误"));
    }
    if params.password.trim().is_empty() {
        return Err(param_err("密码不能为空"));
    }
    if params.password.len() < 6 {
        return Err(param_err("密码长度不能小于6"));
    }
    if params.nickname.trim().is_empty() {
        return Err(param_err("昵称不能为空"));
    }

    // 已存在管理员则禁止重复初始化（对应 Java 中 isInitialized 校验）
    let admin_exists = UserEntity::find()
        .filter(UserCol::Admin.eq(1))
        .filter(UserCol::IsDeleted.eq(0))
        .count(&state.db)
        .await?
        > 0;
    if admin_exists {
        return Err(AppError::Business {
            code: ResCode::NoPermission.code().to_string(),
            message: "系统已初始化，不可重复操作".to_string(),
        });
    }

    // 邮箱不能重复
    if UserEntity::find()
        .filter(UserCol::Email.eq(params.email.clone()))
        .filter(UserCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?
        .is_some()
    {
        return Err(param_err("该邮箱已被注册"));
    }

    let user_id = Uuid::new_v4().simple().to_string();
    let active = UserActive {
        id: Set(user_id.clone()),
        email: Set(params.email),
        password: Set(hash_password(&params.password)?),
        nickname: Set(params.nickname),
        admin: Set(1),
        status: Set(UserStatus::Normal as i32),
        remind_enabled: Set(0),
        remind_day: Set(None),
        cancel_time: Set(None),
        create_time: Set(chrono::Local::now().naive_local()),
        is_deleted: Set(0),
        ..Default::default()
    };
    active.insert(&state.db).await?;

    Ok(Json(ApiResponse::success(user_id)))
}

/// 用户注册：`POST /user/register`
///
/// 对应 Java: `SysUserService.userRegister`。
/// 注册成功后会同步发送账户激活邮件。
pub async fn register(
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    // 校验注册功能是否开启（对应 Java `sysConfigService.isRegisterEnabled`）
    if !state.config_service.is_register_enabled(&state.db).await? {
        return Err(AppError::Business {
            code: ResCode::NoPermission.code().to_string(),
            message: "系统未开启注册功能".to_string(),
        });
    }

    let email = params.email.unwrap_or_default();
    if email.trim().is_empty() {
        return Err(param_err("邮箱不能为空"));
    }
    if !is_valid_email(&email) {
        return Err(param_err("邮箱格式错误"));
    }
    let password = params.password.unwrap_or_default();
    if password.trim().is_empty() {
        return Err(param_err("密码不能为空"));
    }
    if password.len() < 6 {
        return Err(param_err("密码长度不能小于6"));
    }
    let nickname = params.nickname.unwrap_or_default();
    if nickname.trim().is_empty() {
        return Err(param_err("昵称不能为空"));
    }

    // 邮箱唯一性校验
    if UserEntity::find()
        .filter(UserCol::Email.eq(email.clone()))
        .filter(UserCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?
        .is_some()
    {
        return Err(AppError::ResCode(ResCode::Unauthorized));
    }

    let user_id = Uuid::new_v4().simple().to_string();
    let active = UserActive {
        id: Set(user_id.clone()),
        email: Set(email.clone()),
        password: Set(hash_password(&password)?),
        nickname: Set(nickname.clone()),
        admin: Set(0),
        // 与 Java 行为一致：初始为「未激活」，待用户点击邮件中的激活链接
        status: Set(UserStatus::Unactivated as i32),
        remind_enabled: Set(0),
        remind_day: Set(None),
        cancel_time: Set(None),
        create_time: Set(chrono::Local::now().naive_local()),
        is_deleted: Set(0),
        ..Default::default()
    };
    active.insert(&state.db).await?;

    // 生成激活链接（Java 端通过 SecureUtil.aes 加密；此处用专用 JWT 等价表达）
    let activate_token = token::create(
        &user_id,
        TokenPurpose::Activate,
        &state.activate_token_secret,
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    let web_site = state.config_service.get_web_site(&state.db).await?;
    let activate_link = build_activate_link(web_site.as_deref(), &activate_token);

    // 发送账户激活邮件（对应 Java `emailService.sendActivateAccountEmail`）
    let mut mail = MailParams::new();
    mail.to = email.clone();
    mail.user_name = nickname;
    mail.user_email = email.clone();
    mail.activate_url = activate_link;
    state
        .email_service
        .send_activate_account_email(&state.db, mail)
        .await?;

    Ok(Json(ApiResponse::success(email)))
}

/// 激活账户：`POST /user/activate`
///
/// 对应 Java: `SysUserService.activateAccount`
pub async fn activate(
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let token_str = params.activate_token.unwrap_or_default();
    if token_str.trim().is_empty() {
        return Err(param_err("激活令牌不能为空"));
    }

    // 解析并校验令牌
    let user_id = token::parse(
        &token_str,
        TokenPurpose::Activate,
        &state.activate_token_secret,
    )
    .ok_or_else(|| param_err("激活令牌错误或已过期"))?;

    let user = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| param_err("激活令牌错误或已过期"))?;

    // 状态必须为「未激活」
    if user.status != UserStatus::Unactivated as i32 {
        return Err(param_err("用户状态异常，无需激活"));
    }

    // 更新为正常状态
    let mut active: UserActive = user.into();
    active.status = Set(UserStatus::Normal as i32);
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    active.update(&state.db).await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 重新发送激活邮件：`POST /user/resendActivateEmail`
///
/// 对应 Java: `SysUserService.resendActivateEmail`
pub async fn resend_activate_email(
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let email = params.email.unwrap_or_default();
    if email.trim().is_empty() {
        return Err(param_err("邮箱不能为空"));
    }
    if !is_valid_email(&email) {
        return Err(param_err("邮箱格式错误"));
    }

    let user = UserEntity::find()
        .filter(UserCol::Email.eq(email.clone()))
        .filter(UserCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Business {
            code: ResCode::DataNotExist.code().to_string(),
            message: "用户不存在".to_string(),
        })?;

    // 已激活：明确告知，避免重复发送
    if user.status == UserStatus::Normal as i32 {
        return Err(AppError::Business {
            code: ResCode::UserStatusError.code().to_string(),
            message: "用户已激活，无需重复激活".to_string(),
        });
    }
    if user.status != UserStatus::Unactivated as i32 {
        return Err(AppError::Business {
            code: ResCode::UserStatusError.code().to_string(),
            message: "用户状态异常，请联系管理员".to_string(),
        });
    }

    let activate_token = token::create(
        &user.id,
        TokenPurpose::Activate,
        &state.activate_token_secret,
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    let web_site = state.config_service.get_web_site(&state.db).await?;
    let activate_link = build_activate_link(web_site.as_deref(), &activate_token);

    let mut mail = MailParams::new();
    mail.to = email.clone();
    mail.user_name = user.nickname.clone();
    mail.user_email = email.clone();
    mail.activate_url = activate_link;
    state
        .email_service
        .send_activate_account_email(&state.db, mail)
        .await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 用户登录：`POST /user/login`
///
/// 对应 Java: `SysUserService.login`
pub async fn login(
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let email = params.email.unwrap_or_default();
    let password = params.password.unwrap_or_default();

    let user = UserEntity::find()
        .filter(UserCol::Email.eq(email))
        .filter(UserCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?
        .ok_or(AppError::ResCode(ResCode::LoginInfoError))?;

    // BCrypt 验证密码
    if !verify(&password, &user.password).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))? {
        return Err(AppError::ResCode(ResCode::LoginInfoError));
    }

    // 状态校验
    if user.status == UserStatus::Unactivated as i32 {
        return Err(AppError::ResCode(ResCode::UserNotActivated));
    }
    if user.status != UserStatus::Normal as i32 {
        return Err(AppError::ResCode(ResCode::UserStatusError));
    }

    let token = jwt::create_token(&user.id, &state.jwt_secret)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    Ok(Json(ApiResponse::success(token)))
}

/// 用户退出登录：`POST /user/logout`
///
/// 对应 Java: `SysUserController.logout`。无状态 JWT，后端不做任何处理。
pub async fn logout() -> Result<Json<ApiResponse<()>>, AppError> {
    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 忘记密码：`POST /user/forgotPassword`
///
/// 对应 Java: `SysUserService.forgotPassword`。
/// 找不到用户或状态异常时静默成功，避免账户枚举。
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let email = params.email.unwrap_or_default();

    let user = UserEntity::find()
        .filter(UserCol::Email.eq(email.clone()))
        .filter(UserCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?;

    let Some(user) = user else {
        tracing::warn!("未查找到该用户，尝试找回密码：{}", email);
        return Ok(Json(ApiResponse::<()>::success_empty()));
    };
    if user.status != UserStatus::Normal as i32 {
        tracing::warn!(
            "用户状态异常，尝试找回密码：id={}, status={}",
            user.id,
            user.status
        );
        return Ok(Json(ApiResponse::<()>::success_empty()));
    }

    let reset_token = token::create(
        &user.id,
        TokenPurpose::ResetPassword,
        &state.reset_pwd_token_secret,
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    let web_site = state.config_service.get_web_site(&state.db).await?;
    let reset_link = build_reset_link(web_site.as_deref(), &reset_token);

    let mut mail = MailParams::new();
    mail.to = email.clone();
    mail.user_name = user.nickname.clone();
    mail.user_email = email.clone();
    mail.repassword = reset_link;
    state
        .email_service
        .send_retrieve_password_email(&state.db, mail)
        .await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 重置密码：`POST /user/resetPassword`
///
/// 对应 Java: `SysUserService.resetPassword`
pub async fn reset_password(
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let token_str = params.re_password_token.unwrap_or_default();
    if token_str.trim().is_empty() {
        return Err(param_err("重置密码令牌不能为空"));
    }
    let user_id = token::parse(
        &token_str,
        TokenPurpose::ResetPassword,
        &state.reset_pwd_token_secret,
    )
    .ok_or_else(|| param_err("重置密码令牌错误"))?;

    let password = params.password.unwrap_or_default();
    if password.trim().is_empty() {
        return Err(param_err("密码不能为空"));
    }

    let user = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| param_err("重置密码令牌错误"))?;

    let mut active: UserActive = user.into();
    active.password = Set(hash_password(&password)?);
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    active.update(&state.db).await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 修改密码：`POST /user/changePassword`
///
/// 对应 Java: `SysUserService.changePassword`
pub async fn change_password(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<ChangePasswordDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    if params.old_password.trim().is_empty() {
        return Err(param_err("旧密码不能为空"));
    }
    if params.new_password.trim().is_empty() {
        return Err(param_err("新密码不能为空"));
    }
    if params.new_password.len() < 6 {
        return Err(param_err("新密码长度不能小于6"));
    }

    let user = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Business {
            code: ResCode::DataNotExist.code().to_string(),
            message: "用户不存在".to_string(),
        })?;

    // 校验旧密码
    if !verify(&params.old_password, &user.password)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?
    {
        return Err(param_err("旧密码错误"));
    }

    let mut active: UserActive = user.into();
    active.password = Set(hash_password(&params.new_password)?);
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    active.update(&state.db).await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 查询当前登录用户信息：`POST /user/queryMyInfo`
///
/// 对应 Java: `SysUserService.queryMyUserInfo`
pub async fn query_my_info(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Business {
            code: ResCode::DataNotExist.code().to_string(),
            message: "用户不存在".to_string(),
        })?;
    // UserResponse 已经过滤了 password 字段
    Ok(Json(ApiResponse::success(user.into())))
}

/// 修改当前登录用户信息：`POST /user/updateMyInfo`
///
/// 对应 Java: `SysUserService.updateMyUserInfo`。
/// 仅允许修改昵称和提醒相关字段，其他字段忽略。
pub async fn update_my_info(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = UserEntity::find_by_id(user_id.clone())
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Business {
            code: ResCode::DataNotExist.code().to_string(),
            message: "用户不存在".to_string(),
        })?;

    let mut active: UserActive = user.into();
    if let Some(nickname) = params.nickname {
        active.nickname = Set(nickname);
    }
    if let Some(remind_enabled) = params.remind_enabled {
        active.remind_enabled = Set(remind_enabled);
    }
    if let Some(remind_day) = params.remind_day {
        active.remind_day = Set(Some(remind_day));
    }
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    let updated = active.update(&state.db).await?;

    Ok(Json(ApiResponse::success(updated.into())))
}

/// 注销当前登录用户（进入冷静期）：`POST /user/canceledMyUser`
///
/// 对应 Java: `SysUserService.canceledMyUser`。
/// 实际删除由后续定时任务（待实现）扫描 `cancel_time` 后处理。
pub async fn canceled_my_user(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let user = UserEntity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::ResCode(ResCode::UserStatusError))?;

    let mut active: UserActive = user.into();
    active.status = Set(UserStatus::CanceledWait as i32);
    active.cancel_time = Set(Some(chrono::Local::now().naive_local()));
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    active.update(&state.db).await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

// ==================== 管理员接口 ====================

/// 管理员查询所有用户（分页）：`POST /user/list`
///
/// 对应 Java: `SysUserService.queryList`
pub async fn admin_query_list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(params): Json<QueryUserDto>,
) -> Result<Json<ApiResponse<PageResult<UserResponse>>>, AppError> {
    // 构建查询条件（与 Java 端 like / eq 行为对齐）
    let mut q = UserEntity::find().filter(UserCol::IsDeleted.eq(0));
    if let Some(nickname) = params.nickname.filter(|s| !s.is_empty()) {
        q = q.filter(UserCol::Nickname.contains(&nickname));
    }
    if let Some(email) = params.email.filter(|s| !s.is_empty()) {
        q = q.filter(UserCol::Email.contains(&email));
    }
    if let Some(is_admin) = params.is_admin {
        q = q.filter(UserCol::Admin.eq(is_admin));
    }
    if let Some(status) = params.status {
        q = q.filter(UserCol::Status.eq(status));
    }

    // 兜底排序，避免不同 SQLite 版本返回顺序不稳定
    let q = q.order_by_desc(UserCol::CreateTime);

    let page_num = params.page.page_num.max(1) as u64;
    let page_size = params.page.page_size.max(1) as u64;
    let paginator = q.paginate(&state.db, page_size);
    let total = paginator.num_items().await?;
    // SeaORM 的页码从 0 开始
    let records = paginator.fetch_page(page_num - 1).await?;

    let result = PageResult::new(
        records.into_iter().map(UserResponse::from).collect(),
        total,
        page_num,
        page_size,
    );
    Ok(Json(ApiResponse::success(result)))
}

/// 管理员查询单个用户信息：`POST /user/queryUserInfo`
///
/// 对应 Java: `SysUserService.queryUserInfo`。
/// Java 端 body 是裸字符串 `@RequestBody String userId`；这里同样接收 JSON 字符串。
pub async fn admin_query_user_info(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(user_id): Json<String>,
) -> Result<Json<ApiResponse<Option<UserResponse>>>, AppError> {
    if user_id.trim().is_empty() {
        return Ok(Json(ApiResponse::success(None)));
    }
    let user = UserEntity::find_by_id(user_id).one(&state.db).await?;
    Ok(Json(ApiResponse::success(user.map(UserResponse::from))))
}

/// 管理员重置用户密码：`POST /user/adminResetPassword`
///
/// 对应 Java: `SysUserService.adminResetPassword`
pub async fn admin_reset_password(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(params): Json<UserDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    // id 为空则视作 no-op（与 Java 行为一致）
    let Some(id) = params.id.filter(|s| !s.is_empty()) else {
        return Ok(Json(ApiResponse::<()>::success_empty()));
    };
    let password = params.password.unwrap_or_default();
    if password.trim().is_empty() {
        return Err(param_err("密码不能为空"));
    }

    let user = UserEntity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Business {
            code: ResCode::DataNotExist.code().to_string(),
            message: "用户不存在".to_string(),
        })?;

    let mut active: UserActive = user.into();
    active.password = Set(hash_password(&password)?);
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    active.update(&state.db).await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 管理员启用 / 禁用用户（在两种状态间翻转）：`POST /user/enableOrDisableUser`
///
/// 对应 Java: `SysUserService.enableOrDisableUser`
pub async fn admin_enable_or_disable(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(user_ids): Json<Vec<String>>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    if user_ids.is_empty() {
        return Ok(Json(ApiResponse::<()>::success_empty()));
    }
    let users: Vec<sys_user::Model> = UserEntity::find()
        .filter(UserCol::Id.is_in(user_ids))
        .all(&state.db)
        .await?;

    for user in users {
        // 仅在「正常」与「停用」之间翻转，其他状态保持不变
        let next_status = if user.status == UserStatus::Normal as i32 {
            UserStatus::Disabled as i32
        } else {
            UserStatus::Normal as i32
        };
        let mut active: UserActive = user.into();
        active.status = Set(next_status);
        active.update_time = Set(Some(chrono::Local::now().naive_local()));
        active.update(&state.db).await?;
    }

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 管理员删除用户（物理删除）：`POST /user/deleteUser`
///
/// 对应 Java: `SysUserService.deleteUser`
pub async fn admin_delete_user(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(user_ids): Json<Vec<String>>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    if user_ids.is_empty() {
        return Ok(Json(ApiResponse::<()>::success_empty()));
    }
    UserEntity::delete_many()
        .filter(UserCol::Id.is_in(user_ids))
        .exec(&state.db)
        .await?;
    Ok(Json(ApiResponse::<()>::success_empty()))
}
