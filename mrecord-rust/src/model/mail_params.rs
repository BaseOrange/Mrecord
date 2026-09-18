//! 邮件参数 BO
//!
//! 对应 Java: `com.dcz.mrecord.bo.MailParamsBO`
//!
//! Java 版本通过 `SpringUtil.getBean(SysConfigService.class)` 在 `getParams()`
//! 中懒加载站点地址、管理员邮箱。Rust 端没有静态容器可拉，由调用方传入这两个值，
//! 在 [`crate::service::email::EmailService`] 渲染模板前注入到占位符 Map 中。

use std::collections::HashMap;

use chrono::{Datelike, Local};

/// 邮件渲染参数
///
/// 字段命名与 Java `MailParamsBO` 对齐，一一对应模板里的 `${MR-XXX}` 占位符。
#[derive(Default, Clone)]
pub struct MailParams {
    /// 接收者邮箱（用于 `setTo`）
    pub to: String,
    /// 用户昵称 → 模板 `${MR-UserName}`
    pub user_name: String,
    /// 用户邮箱 → 模板 `${MR-UserEmail}`
    pub user_email: String,
    /// 注册时间 → 模板 `${MR-RegisterDate}`（默认当前时间，"yyyy-MM-dd HH:mm"）
    pub register_date: String,
    /// 当前年月（仅月份，例如 "6"）→ 模板 `${MR-YearMonth}`
    pub curr_year_month: String,
    /// 当前年份（如 "2026"）→ 模板 `${MR-Year}`
    pub curr_year: String,
    /// 重置密码链接 → 模板 `${MR-Repassword}`
    pub repassword: String,
    /// 账户激活链接 → 模板 `${MR-ActivateUrl}`
    pub activate_url: String,
    /// 导出文件名 → 模板 `${MR-FileName}`
    pub file_name: String,
    // ==================== 年度总结邮件专用（对应 Java `MailParamsBO` 同名字段）====================
    /// 总结年份（上一年，如 "2025"）→ 模板 `${MR-SummaryYear}`
    pub summary_year: String,
    /// 记账月数 → 模板 `${MR-RecordedMonths}`
    pub recorded_months: String,
    /// 账簿数量 → 模板 `${MR-BookCount}`
    pub book_count: String,
    /// 年度明细记录条数 → 模板 `${MR-ItemCount}`
    pub item_count: String,
    /// 年末总资产 → 模板 `${MR-TotalAsset}`
    pub total_asset: String,
    /// 年末总负债 → 模板 `${MR-TotalLiability}`
    pub total_liability: String,
    /// 年末净资产 → 模板 `${MR-NetAsset}`
    pub net_asset: String,
    /// 年初净资产 → 模板 `${MR-NetAssetStart}`
    pub net_asset_start: String,
    /// 净资产变化金额（带符号）→ 模板 `${MR-NetAssetChange}`
    pub net_asset_change: String,
    /// 净资产变化率（带符号 %，年初为零无可比基准时为 "—"）→ 模板 `${MR-NetAssetChangeRate}`
    pub net_asset_change_rate: String,
}

impl MailParams {
    /// 创建一个带默认时间字段的实例（其他字段均为空字符串）
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            register_date: now.format("%Y-%m-%d %H:%M").to_string(),
            curr_year_month: now.month().to_string(),
            curr_year: now.year().to_string(),
            ..Default::default()
        }
    }

    /// 转为模板占位符 Map
    ///
    /// 对应 Java `MailParamsBO.getParams()`，但 `web_site` 与 `admin_mail` 由调用方
    /// 传入（这两个值在 Java 中通过 `SpringUtil.getBean` 现取，Rust 端避免隐式依赖）。
    pub fn to_placeholders(
        &self,
        web_site: Option<&str>,
        admin_mail: Option<&str>,
    ) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("MR-UserName".to_string(), self.user_name.clone());
        m.insert("MR-UserEmail".to_string(), self.user_email.clone());
        m.insert("MR-YearMonth".to_string(), self.curr_year_month.clone());
        m.insert("MR-Year".to_string(), self.curr_year.clone());
        m.insert("MR-WebSite".to_string(), web_site.unwrap_or("").to_string());
        m.insert(
            "MR-AdminMail".to_string(),
            admin_mail.unwrap_or("").to_string(),
        );
        m.insert("MR-Repassword".to_string(), self.repassword.clone());
        m.insert("MR-ActivateUrl".to_string(), self.activate_url.clone());
        m.insert("MR-FileName".to_string(), self.file_name.clone());
        m.insert("MR-RegisterDate".to_string(), self.register_date.clone());
        // 年度总结占位符
        m.insert("MR-SummaryYear".to_string(), self.summary_year.clone());
        m.insert(
            "MR-RecordedMonths".to_string(),
            self.recorded_months.clone(),
        );
        m.insert("MR-BookCount".to_string(), self.book_count.clone());
        m.insert("MR-ItemCount".to_string(), self.item_count.clone());
        m.insert("MR-TotalAsset".to_string(), self.total_asset.clone());
        m.insert(
            "MR-TotalLiability".to_string(),
            self.total_liability.clone(),
        );
        m.insert("MR-NetAsset".to_string(), self.net_asset.clone());
        m.insert("MR-NetAssetStart".to_string(), self.net_asset_start.clone());
        m.insert(
            "MR-NetAssetChange".to_string(),
            self.net_asset_change.clone(),
        );
        m.insert(
            "MR-NetAssetChangeRate".to_string(),
            self.net_asset_change_rate.clone(),
        );
        m
    }
}
