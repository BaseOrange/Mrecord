//! 金额精度工具
//!
//! 对应 Java `BigDecimal` 金额处理约定，统一使用两位小数和 HALF_UP 舍入。

use rust_decimal::{Decimal, RoundingStrategy};

/// 金额和增长率统一保留的小数位数。
pub const MONEY_SCALE: u32 = 2;

/// 将金额按 Java BigDecimal HALF_UP 规则保留两位小数。
pub fn round_money(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::MidpointAwayFromZero)
}

/// 构造已按金额规则舍入的零值。
pub fn zero_money() -> Decimal {
    round_money(Decimal::ZERO)
}
