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

/// 将金额格式化为带千分位分隔符的展示字符串（两位小数，HALF_UP）。
///
/// 仅供邮件模板等「给人看」的场景使用（对应 Java 邮件侧 `BigDecimal.setScale(2, HALF_UP)`
/// 后的展示格式）；接口响应仍应使用 [`serialize_decimal_as_number`] 输出 JSON 数字。
///
/// - 正数：`1,234,567.89`
/// - 负数：`-1,234.50`
/// - 零：`0.00`（`-0.00x` 舍入后也输出 `0.00`，不出现负号）
pub fn format_money(value: Decimal) -> String {
    use rust_decimal::prelude::ToPrimitive;

    let rounded = round_money(value);
    let cents = (rounded.abs() * Decimal::from(100))
        .round_dp(0)
        .to_i64()
        .unwrap_or(0);
    let sign = if rounded.is_sign_negative() && cents != 0 {
        "-"
    } else {
        ""
    };
    format!(
        "{}{}.{:02}",
        sign,
        group_thousands(cents / 100),
        cents % 100
    )
}

/// 为整数部分每三位插入千分位分隔符。
fn group_thousands(value: i64) -> String {
    let digits = value.abs().to_string();
    let bytes = digits.as_bytes();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*byte as char);
    }
    out
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

/// 从 JSON 数字或字符串解析**可空**金额（缺失或 `null` 解析为 `None`）。
///
/// 供请求 DTO 的 `#[serde(default, deserialize_with = "...")]` 使用，把「金额缺失」
/// 从反序列化的裸 400 错误转为可由业务层统一校验的 `Option`（对应 Java
/// `checkFinItemList` 中 `itemValue == null` 的判断），字段本身缺失时由
/// `#[serde(default)]` 兜底为 `None`。
pub fn deserialize_optional_decimal_from_number_or_string<'de, D>(
    deserializer: D,
) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Number(n) => Decimal::from_str_exact(&n.to_string())
            .map(Some)
            .map_err(serde::de::Error::custom),
        serde_json::Value::String(s) => Decimal::from_str_exact(&s)
            .map(Some)
            .map_err(serde::de::Error::custom),
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

    /// 模拟可空金额字段：缺失 / null → None，其余走宽松解析
    #[derive(Deserialize)]
    struct OptionalMoneyRequest {
        #[serde(
            default,
            deserialize_with = "deserialize_optional_decimal_from_number_or_string"
        )]
        value: Option<Decimal>,
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
    fn optional_decimal_accepts_missing_null_number_and_string() {
        // 字段缺失 → None（由 #[serde(default)] 兜底，对应 Java itemValue 未传）
        let missing: OptionalMoneyRequest = serde_json::from_str("{}").unwrap();
        assert!(missing.value.is_none());

        // 显式 null → None（对应 Java checkFinItemList 的 itemValue == null 分支）
        let null_val: OptionalMoneyRequest = serde_json::from_str(r#"{"value":null}"#).unwrap();
        assert!(null_val.value.is_none());

        // JSON 数字 → Some
        let from_number: OptionalMoneyRequest =
            serde_json::from_str(r#"{"value":123.45}"#).unwrap();
        assert_eq!(from_number.value, Some(Decimal::new(12345, 2)));

        // 字符串金额 → Some（兼容旧接口）
        let from_string: OptionalMoneyRequest =
            serde_json::from_str(r#"{"value":"123.45"}"#).unwrap();
        assert_eq!(from_string.value, Some(Decimal::new(12345, 2)));

        // 非数字 / 非字符串 / 非 null 仍报错
        assert!(serde_json::from_str::<OptionalMoneyRequest>(r#"{"value":true}"#).is_err());
    }

    // ==================== format_money 测试 ====================
    //
    // 仅供邮件等展示场景：两位小数 + 千分位分隔符，对齐 Java BigDecimal 的展示格式。

    #[test]
    fn format_money_groups_thousands_and_keeps_two_decimals() {
        // 常规正数：整数部分每三位一个逗号，小数强制两位
        assert_eq!(format_money(Decimal::from(1234567)), "1,234,567.00");
        assert_eq!(format_money(d(123456789, 2)), "1,234,567.89");
        // 多级千分位
        assert_eq!(format_money(Decimal::from(1234567890)), "1,234,567,890.00");
        // 整数也补齐两位小数
        assert_eq!(format_money(Decimal::from(100)), "100.00");
        // 小于 1 的金额不需要千分位
        assert_eq!(format_money(d(5, 2)), "0.05");
    }

    #[test]
    fn format_money_handles_zero_and_negatives() {
        assert_eq!(format_money(Decimal::ZERO), "0.00");
        // 负数带符号
        assert_eq!(format_money(d(-123450, 2)), "-1,234.50");
        // 舍入后为零的负数不应出现 "-0.00"
        assert_eq!(format_money(d(-4, 3)), "0.00");
        assert_eq!(format_money(d(-1, 2)), "-0.01");
    }

    #[test]
    fn format_money_rounds_half_up() {
        // 第三位小数 5 → 进位（MidpointAwayFromZero）
        assert_eq!(format_money(d(123455, 3)), "123.46");
        // 第三位小数 4 → 舍去
        assert_eq!(format_money(d(123454, 3)), "123.45");
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
