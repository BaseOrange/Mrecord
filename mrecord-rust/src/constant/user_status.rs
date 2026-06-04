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
    Canceled = 3,
    /// 未激活
    Unactivated = 4,
}

impl UserStatus {
    /// 从数据库整数值还原，非法值返回 None
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Normal),
            1 => Some(Self::Disabled),
            2 => Some(Self::CanceledWait),
            3 => Some(Self::Canceled),
            4 => Some(Self::Unactivated),
            _ => None,
        }
    }
}
