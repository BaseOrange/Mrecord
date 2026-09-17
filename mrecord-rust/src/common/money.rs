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

/// 将金额按 Java BigDecimal HALF_UP 规则保留两位小数。
pub fn round_money(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::MidpointAwayFromZero)
}

/// 构造已按金额规则舍入的零值。
pub fn zero_money() -> Decimal {
    round_money(Decimal::ZERO)
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
}
