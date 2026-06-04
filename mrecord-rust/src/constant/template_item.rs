//! 账簿模板项类型常量
//!
//! 对应 Java: `com.dcz.mrecord.constant.TempItemTypeConst`

/// 模板项类型（MR_ITEM_TYPE 列取值）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TempItemType {
    /// 负债
    Liability = -1,
    /// 不统计仅记录
    OnlyRecord = 0,
    /// 资产
    Asset = 1,
}

impl TempItemType {
    /// 从数据库整数值还原，非法值返回 None
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::Liability),
            0 => Some(Self::OnlyRecord),
            1 => Some(Self::Asset),
            _ => None,
        }
    }
}
