//! 金额精度与序列化工具
//!
//! 对应 Java `BigDecimal` 金额处理约定，统一使用两位小数和 HALF_UP 舍入；
//! JSON 序列化对齐 Java Jackson 对 `BigDecimal` 的默认行为——输出 **JSON 数字**，
//! 与前端 `totalAsset?: number` 等类型约定保持一致（输出字符串会导致前端
//! `formatMoney` / `reduce` 求和 / 图表渲染出现 NaN 或字符串拼接）。

use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 金额和增长率统一保留的小数位数。
pub const MONEY_SCALE: u32 = 2;

/// 增长率中间除法保留的小数位数（对齐 Java `divide(scale=4)`）。
pub const GROWTH_RATE_INTERMEDIATE_SCALE: u32 = 4;

/// 将金额按 Java BigDecimal HALF_UP 规则保留两位小数。
pub fn round_money(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::MidpointAwayFromZero)
}

/// 构造已按金额规则舍入的零值。
pub fn zero_money() -> Decimal {
    round_money(Decimal::ZERO)
}

/// 计算环比/同比增长率，舍入链路对齐 Java:
///
/// `(cur - base).divide(|base|, 4, HALF_UP).multiply(100).setScale(2, HALF_UP)`
///
/// 对应 Java: `FinMonthRecordServiceImpl.getMonthOnMonthVal` / `getYearOnYearVal`
///
/// - 分母为 0 时返回 0.00（不计算增长率）
/// - 中间除法显式保留 4 位小数 + `MidpointAwayFromZero`（对齐 Java `divide(scale=4, HALF_UP)`）
/// - 最终结果保留 2 位小数 + `MidpointAwayFromZero`（对齐 Java `setScale(2, HALF_UP)`）
///
/// 返回百分比值（如 10.50 表示 10.50%）
pub fn calculate_growth_rate(current: Decimal, base: Decimal) -> Decimal {
    if base.is_zero() {
        return zero_money();
    }
    let diff = current - base;
    let ratio = diff / base.abs();
    let ratio_rounded = ratio.round_dp_with_strategy(
        GROWTH_RATE_INTERMEDIATE_SCALE,
        RoundingStrategy::MidpointAwayFromZero,
    );
    let percentage = ratio_rounded * Decimal::from(100);
    percentage.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::MidpointAwayFromZero)
}

/// 将金额序列化为 JSON 数字（等价 Java Jackson 对 `BigDecimal` 的默认输出）
///
/// 供响应 DTO 的 `#[serde(serialize_with = "...")]` 使用。优先取 f64 输出数字；
/// 仅当金额超出 f64 表示范围（对记账场景不可能出现）时退回字符串，
/// 相比 `rust_decimal::serde::float` 内部的 `to_f64().unwrap()` 更稳健，不会 panic。
pub fn serialize_decimal_as_number<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use rust_decimal::prelude::ToPrimitive;
    match value.to_f64() {
        Some(float_value) => float_value.serialize(serializer),
        None => value.to_string().serialize(serializer),
    }
}

/// 从 JSON 数字或字符串解析金额（宽松反序列化）
///
/// 供请求 DTO 的 `#[serde(deserialize_with = "...")]` 使用。等价 Java `BigDecimal`
/// 对 JSON 数值的解析行为，同时兼容旧接口可能传入的字符串金额。
pub fn deserialize_decimal_from_number_or_string<'de, D>(
    deserializer: D,
) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(n) => {
            Decimal::from_str_exact(&n.to_string()).map_err(serde::de::Error::custom)
        }
        serde_json::Value::String(s) => {
            Decimal::from_str_exact(&s).map_err(serde::de::Error::custom)
        }
        other => Err(serde::de::Error::custom(format!(
            "金额必须是数字或字符串: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// 模拟响应 DTO 的金额字段，验证 `serialize_with` 的真实接入路径
    #[derive(Serialize)]
    struct MoneyResponse {
        #[serde(serialize_with = "serialize_decimal_as_number")]
        value: Decimal,
    }

    /// 无注解的金额字段：验证 `Decimal` 的**默认**序列化也是 JSON 数字
    #[derive(Serialize)]
    struct DefaultDecimal {
        value: Decimal,
    }

    /// 模拟请求 DTO 的金额字段，验证 `deserialize_with` 的真实接入路径
    #[derive(Deserialize)]
    struct MoneyRequest {
        #[serde(deserialize_with = "deserialize_decimal_from_number_or_string")]
        value: Decimal,
    }

    #[test]
    fn serialize_outputs_json_number() {
        // 响应金额必须是 JSON 数字（无引号），否则前端 number 类型与图表渲染会失效
        let json = serde_json::to_value(MoneyResponse {
            value: Decimal::new(12345, 2),
        })
        .unwrap();
        assert_eq!(json, json!({"value": 123.45}));
        assert!(json["value"].is_number());

        // 负数、零、整数均应为数字
        for value in [
            Decimal::new(-505, 2), // -5.05
            Decimal::ZERO,
            Decimal::from(1000),
        ] {
            let json = serde_json::to_value(MoneyResponse { value }).unwrap();
            assert!(
                json["value"].is_number(),
                "金额必须序列化为 JSON 数字: {value:?} -> {json}"
            );
        }
    }

    #[test]
    fn default_decimal_serialization_is_number() {
        // 项目级约定：未写 `#[serde(serialize_with = ...)]` 的金额字段也必须是 JSON 数字。
        // 该行为由 Cargo.toml 中 `rust_decimal` 的 `serde-float` feature 保证；
        // 若该 feature 被移除，本测试失败并提示恢复方式。
        let json = serde_json::to_value(DefaultDecimal {
            value: Decimal::new(12345, 2),
        })
        .unwrap();
        assert!(
            json["value"].is_number(),
            "Decimal 默认序列化必须是 JSON 数字（需在 Cargo.toml 保留 rust_decimal 的 `serde-float` feature）: {json}"
        );
    }

    #[test]
    fn deserialize_accepts_number_and_string() {
        // 前端传入 JSON 数字
        let from_number: MoneyRequest = serde_json::from_str(r#"{"value":123.45}"#).unwrap();
        assert_eq!(from_number.value, Decimal::new(12345, 2));

        // 兼容旧接口的字符串金额
        let from_string: MoneyRequest = serde_json::from_str(r#"{"value":"123.45"}"#).unwrap();
        assert_eq!(from_string.value, Decimal::new(12345, 2));

        // 既非数字也非字符串应报错
        assert!(serde_json::from_str::<MoneyRequest>(r#"{"value":null}"#).is_err());
    }

    // ==================== calculate_growth_rate 测试 ====================
    //
    // 对应 Java: FinMonthRecordServiceImpl.getMonthOnMonthVal / getYearOnYearVal
    // 舍入链路: (cur - base).divide(|base|, 4, HALF_UP).multiply(100).setScale(2, HALF_UP)
    //
    // 测试覆盖：
    // - 分母为 0
    // - 零增长（current == base）
    // - 正增长 / 负增长
    // - 负净资产（正负基数、正负当期）
    // - 半数边界（12.5%、12.555% 等）
    // - 微小增长（中间除法舍入影响）

    /// 构造 Decimal 的辅助函数（整数 / 10^scale）
    fn d(val: i64, scale: u32) -> Decimal {
        Decimal::new(val, scale)
    }

    #[test]
    fn growth_rate_zero_base_returns_zero() {
        // 分母为 0 时返回 0.00，不计算增长率
        assert_eq!(
            calculate_growth_rate(d(100, 0), Decimal::ZERO),
            Decimal::new(0, 2)
        );
        assert_eq!(
            calculate_growth_rate(Decimal::ZERO, Decimal::ZERO),
            Decimal::new(0, 2)
        );
        assert_eq!(
            calculate_growth_rate(d(-50, 0), Decimal::ZERO),
            Decimal::new(0, 2)
        );
    }

    #[test]
    fn growth_rate_no_change_returns_zero() {
        // current == base 时增长率为 0.00
        assert_eq!(
            calculate_growth_rate(d(100, 0), d(100, 0)),
            Decimal::new(0, 2)
        );
        assert_eq!(
            calculate_growth_rate(d(-100, 0), d(-100, 0)),
            Decimal::new(0, 2)
        );
    }

    #[test]
    fn growth_rate_positive_growth() {
        // 正增长：110 vs 100 → (10 / 100) = 0.1 → 0.1 * 100 = 10.00
        assert_eq!(
            calculate_growth_rate(d(110, 0), d(100, 0)),
            Decimal::new(1000, 2)
        );
        // 50% 增长
        assert_eq!(
            calculate_growth_rate(d(150, 0), d(100, 0)),
            Decimal::new(5000, 2)
        );
        // 100% 增长
        assert_eq!(
            calculate_growth_rate(d(200, 0), d(100, 0)),
            Decimal::new(10000, 2)
        );
    }

    #[test]
    fn growth_rate_negative_growth() {
        // 负增长：90 vs 100 → (-10 / 100) = -0.1 → -0.1 * 100 = -10.00
        assert_eq!(
            calculate_growth_rate(d(90, 0), d(100, 0)),
            Decimal::new(-1000, 2)
        );
        // -50% 增长
        assert_eq!(
            calculate_growth_rate(d(50, 0), d(100, 0)),
            Decimal::new(-5000, 2)
        );
    }

    #[test]
    fn growth_rate_negative_base_uses_abs() {
        // Java 使用 base.abs() 做分母，避免负数导致增长方向反转
        // 50 vs -100 → (50 - (-100)) / |-100| = 150 / 100 = 1.5 → 150.00
        assert_eq!(
            calculate_growth_rate(d(50, 0), d(-100, 0)),
            Decimal::new(15000, 2)
        );
        // 负负：-50 vs -100 → (-50 - (-100)) / |-100| = 50 / 100 = 0.5 → 50.00
        assert_eq!(
            calculate_growth_rate(d(-50, 0), d(-100, 0)),
            Decimal::new(5000, 2)
        );
        // 负正：-50 vs 100 → (-50 - 100) / |100| = -150 / 100 = -1.5 → -150.00
        assert_eq!(
            calculate_growth_rate(d(-50, 0), d(100, 0)),
            Decimal::new(-15000, 2)
        );
    }

    #[test]
    fn growth_rate_half_up_boundary_12_5() {
        // 12.5% 边界：112.5 vs 100
        // diff = 12.5, ratio = 0.125, ratio_rounded = 0.125, percentage = 12.5, result = 12.50
        assert_eq!(
            calculate_growth_rate(d(1125, 1), d(100, 0)),
            Decimal::new(1250, 2)
        );
    }

    #[test]
    fn growth_rate_half_up_boundary_12_555() {
        // 12.555% 边界：112.555 vs 100
        // diff = 12.555, ratio = 0.12555, ratio_rounded = 0.1256 (5th digit 5, HALF_UP)
        // percentage = 12.56, result = 12.56
        assert_eq!(
            calculate_growth_rate(d(112555, 3), d(100, 0)),
            Decimal::new(1256, 2)
        );
    }

    #[test]
    fn growth_rate_half_up_boundary_12_554() {
        // 12.554% 边界：112.554 vs 100
        // diff = 12.554, ratio = 0.12554, ratio_rounded = 0.1255 (5th digit 4, rounds down)
        // percentage = 12.55, result = 12.55
        assert_eq!(
            calculate_growth_rate(d(112554, 3), d(100, 0)),
            Decimal::new(1255, 2)
        );
    }

    #[test]
    fn growth_rate_half_up_boundary_12_5555() {
        // 12.5555% 边界：112.5555 vs 100
        // diff = 12.5555, ratio = 0.125555, ratio_rounded = 0.1256 (5th digit 5, HALF_UP)
        // percentage = 12.56, result = 12.56
        assert_eq!(
            calculate_growth_rate(d(1125555, 4), d(100, 0)),
            Decimal::new(1256, 2)
        );
    }

    #[test]
    fn growth_rate_tiny_increase_above_threshold() {
        // 0.005% 增长：100.005 vs 100
        // diff = 0.005, ratio = 0.00005, ratio_rounded = 0.0001 (5th digit 5, HALF_UP)
        // percentage = 0.01, result = 0.01
        assert_eq!(
            calculate_growth_rate(d(100005, 3), d(100, 0)),
            Decimal::new(1, 2)
        );
    }

    #[test]
    fn growth_rate_tiny_increase_below_threshold() {
        // 0.004% 增长：100.004 vs 100
        // diff = 0.004, ratio = 0.00004, ratio_rounded = 0.0000 (5th digit 4, rounds down)
        // percentage = 0.00, result = 0.00
        assert_eq!(
            calculate_growth_rate(d(100004, 3), d(100, 0)),
            Decimal::new(0, 2)
        );
    }

    #[test]
    fn growth_rate_intermediate_rounding_precision() {
        // 验证中间除法 scale=4 舍入的正确性
        // current = 100.1256, base = 100
        // diff = 0.1256, ratio = 0.001256, 5th digit is 5 → HALF_UP → 0.0013
        // percentage = 0.13, result = 0.13
        assert_eq!(
            calculate_growth_rate(d(1001256, 4), d(100, 0)),
            Decimal::new(13, 2)
        );
        // current = 100.1249, base = 100
        // diff = 0.1249, ratio = 0.001249, 5th digit is 4 → rounds down → 0.0012
        // percentage = 0.12, result = 0.12
        assert_eq!(
            calculate_growth_rate(d(1001249, 4), d(100, 0)),
            Decimal::new(12, 2)
        );
    }

    #[test]
    fn growth_rate_negative_half_boundary() {
        // 负数增长率的半数边界
        // current = 87.445, base = 100
        // diff = -12.555, ratio = -0.12555, ratio_rounded = -0.1256 (HALF_UP, away from zero)
        // percentage = -12.56, result = -12.56
        assert_eq!(
            calculate_growth_rate(d(87445, 3), d(100, 0)),
            Decimal::new(-1256, 2)
        );
        // current = 87.446, base = 100
        // diff = -12.546, ratio = -0.12546, ratio_rounded = -0.1255 (5th digit 6 → round up away from zero)
        // Wait: -0.12546 → 5th digit is 4? No, 0.12546 has digits 1,2,5,4,6. 5th digit is 6, rounds up.
        // But for negative numbers, HALF_UP means away from zero, so -0.12546 → -0.1255
        // percentage = -12.55, result = -12.55
        assert_eq!(
            calculate_growth_rate(d(87446, 3), d(100, 0)),
            Decimal::new(-1255, 2)
        );
    }

    #[test]
    fn growth_rate_decimal_input_values() {
        // 输入带小数的净资产值
        // current = 105.67, base = 100.00 → diff = 5.67, ratio = 0.0567, rounded = 0.0567, *100 = 5.67
        assert_eq!(
            calculate_growth_rate(d(10567, 2), d(10000, 2)),
            Decimal::new(567, 2)
        );
        // current = 95.33, base = 100.00 → diff = -4.67, ratio = -0.0467, rounded = -0.0467, *100 = -4.67
        assert_eq!(
            calculate_growth_rate(d(9533, 2), d(10000, 2)),
            Decimal::new(-467, 2)
        );
    }
}
