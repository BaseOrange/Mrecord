//! 用户状态常量
//!
//! 对应 Java: `com.dcz.mrecord.constant.UserStatusConst`

/// 用户状态（MR_STATUS 列取值）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UserStatus {
    /// 正常
    Normal = 0,
    /// 停用
    Disabled = 1,
    /// 注销待生效（15 天冷静期）
    CanceledWait = 2,
    /// 已注销
    ///
    /// 保留以对齐 Java `UserStatusConst.CANCELED = 3`：当前注销流程在冷静期结束后
    /// 直接物理删除用户（见 `service::cancel_cleanup_task`），不会写入该状态；
    /// 该取值作为保留的领域值存在，暂无构造点。
    #[allow(dead_code)]
    Canceled = 3,
    /// 未激活
    Unactivated = 4,
}
