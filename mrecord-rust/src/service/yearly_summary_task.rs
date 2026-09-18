//! 年度总结邮件定时任务
//!
//! 对应 Java: `com.dcz.mrecord.task.YearlySummaryTask`（Java 端 `EmailService.sendNewYearReminderEmail`
//! 原本无任何调用入口，本任务为两侧共同补齐的业务定义）。
//!
//! 产品决策（见 `REFACTOR_TODO.md` 3.12）：不做「新财年提醒」，改为每年 1 月 1 日
//! 08:08（本地时间）为用户发送**上一年**的年度财务总结邮件。
//!
//! 触发时机说明：任务在每年 1 月 1 日执行，此时 `Local::now()` 的年份已是新年份，
//! 因此总结年份 = 当前年份 - 1（如 2026-01-01 08:08 总结 2025 年全年数据）。
//!
//! 收件人筛选与月度提醒（[`crate::service::monthly_reminder_task`]）一致：
//! 状态正常、未注销、且已开启邮件提醒（`remind_enabled = 1`）的用户——尊重用户对
//! 邮件提醒的开关意愿。若将来产品需要独立的「年度总结开关」，需加字段 + 前端入口。
//!
//! 跳过规则：用户在册账簿为空，或该年度没有任何月度汇总记录时不发送
//! （「0 个月记账」的空洞总结体验更差）。

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use chrono::{Datelike, Local};
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    common::money::{calculate_growth_rate, format_money},
    constant::user_status::UserStatus,
    entity::fin_book::{Column as BookCol, Entity as BookEntity},
    entity::fin_month_item_record::{Column as ItemCol, Entity as ItemEntity},
    entity::fin_month_record::{
        Column as MonthRecordCol, Entity as MonthRecordEntity, Model as MonthRecordModel,
    },
    entity::sys_user::{Column as UserCol, Entity as UserEntity, Model as UserModel},
    error::AppError,
    model::mail_params::MailParams,
    service::email::EmailService,
    util::schedule::duration_until_yearly,
};

/// 每年触发日期：1 月 1 日（本地时间 08:08，与月度提醒同一时点，寓意「发发」）。
const RUN_MONTH: u32 = 1;
const RUN_DAY: u32 = 1;
const RUN_HOUR: u32 = 8;
const RUN_MINUTE: u32 = 8;

/// 单个用户上一年度的财务总结（由 `FIN_MONTH_RECORD` / `FIN_MONTH_ITEM_RECORD` 聚合而来）。
struct YearSummary {
    /// 总结年份
    year: i32,
    /// 在册账簿数量
    book_count: usize,
    /// 年度明细记录条数
    item_count: u64,
    /// 有月度汇总记录的月份数（跨账簿按月去重）
    recorded_months: usize,
    /// 年初（首个有记录月份）净资产合计
    net_asset_start: Decimal,
    /// 年末（最后有记录月份）净资产合计
    net_asset_end: Decimal,
    /// 年末总资产合计
    total_asset_end: Decimal,
    /// 年末总负债合计
    total_liability_end: Decimal,
}

/// 年度总结邮件任务服务。
///
/// 负责：筛选收件人、聚合用户年度数据、调用 [`EmailService`] 发送年度总结邮件。
pub struct YearlySummaryTask {
    email_service: Arc<EmailService>,
}

impl YearlySummaryTask {
    /// 创建年度总结任务服务实例。
    pub fn new(email_service: Arc<EmailService>) -> Arc<Self> {
        Arc::new(Self { email_service })
    }

    /// 启动后台定时循环：每年 1 月 1 日 08:08 执行一次。
    ///
    /// 对应 Java `@Scheduled(cron = "0 8 8 1 1 ?")`。服务重启安全：任一时刻的下次触发
    /// 时刻唯一，不会因重启在同一天重复发送。
    pub fn start(self: Arc<Self>, db: DatabaseConnection) {
        tokio::spawn(async move {
            loop {
                let wait = duration_until_yearly(RUN_MONTH, RUN_DAY, RUN_HOUR, RUN_MINUTE);
                tracing::info!("年度总结任务将在 {} 秒后执行", wait.as_secs());
                tokio::time::sleep(wait).await;

                if let Err(e) = self.run_once(&db).await {
                    tracing::error!("年度总结任务执行失败: {:?}", e);
                }
            }
        });
    }

    /// 执行一次年度总结发送，返回成功处理的邮件数。
    ///
    /// 「成功处理」含未配置邮箱被优雅跳过的情形（与月度提醒一致）；单个用户失败
    /// 只记日志并跳过，不影响其它用户。
    pub async fn run_once(&self, db: &DatabaseConnection) -> Result<usize, AppError> {
        // 每年 1 月 1 日执行，总结年份为刚结束的一年
        let summary_year = Local::now().year() - 1;
        let users = load_summary_users(db).await?;
        tracing::info!(
            year = summary_year,
            users = users.len(),
            "年度总结任务开始处理"
        );

        let mut sent = 0usize;
        for user in &users {
            match build_year_summary(db, user, summary_year).await {
                Ok(Some(summary)) => {
                    let params = build_yearly_mail_params(user, summary);
                    match self.email_service.send_year_summary_email(db, params).await {
                        Ok(()) => sent += 1,
                        Err(e) => tracing::error!(
                            user_id = %user.id, email = %user.email,
                            "年度总结邮件发送失败: {:?}", e
                        ),
                    }
                }
                Ok(None) => tracing::debug!(
                    user_id = %user.id,
                    "用户 {} 年无月度汇总记录，跳过年度总结",
                    summary_year
                ),
                Err(e) => tracing::error!(
                    user_id = %user.id,
                    "为用户生成年度总结失败，已跳过: {:?}", e
                ),
            }
        }

        tracing::info!(year = summary_year, sent, "年度总结任务执行完成");
        Ok(sent)
    }
}

/// 查询可接收年度总结的用户：正常、未删除、已开启邮件提醒。
///
/// 与 `monthly_reminder_task::load_remind_users` 的用户筛选条件保持一致，
/// 遵循用户对邮件提醒的开关意愿。
async fn load_summary_users(db: &DatabaseConnection) -> Result<Vec<UserModel>, AppError> {
    Ok(UserEntity::find()
        .filter(UserCol::Status.eq(UserStatus::Normal as i32))
        .filter(UserCol::IsDeleted.eq(0))
        .filter(UserCol::RemindEnabled.eq(1))
        .all(db)
        .await?)
}

/// 聚合用户在指定年度的财务数据，生成年度总结。
///
/// 返回 `None` 的情况：用户没有在册账簿，或该年度没有任何月度汇总记录
/// （跳过空洞总结，见模块文档）。
async fn build_year_summary(
    db: &DatabaseConnection,
    user: &UserModel,
    year: i32,
) -> Result<Option<YearSummary>, AppError> {
    let books = BookEntity::find()
        .filter(BookCol::UserId.eq(user.id.clone()))
        .filter(BookCol::IsDeleted.eq(0))
        .all(db)
        .await?;
    if books.is_empty() {
        return Ok(None);
    }
    let book_count = books.len();
    let book_ids: Vec<String> = books.into_iter().map(|book| book.id).collect();

    let records = MonthRecordEntity::find()
        .filter(MonthRecordCol::UserId.eq(user.id.clone()))
        .filter(MonthRecordCol::Year.eq(year))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .order_by_asc(MonthRecordCol::Month)
        .order_by_asc(MonthRecordCol::UpdateTime)
        .all(db)
        .await?;
    if records.is_empty() {
        return Ok(None);
    }

    // 同一（账簿, 月份）可能存在历史 Java 数据产生的重复行（Rust 端 upsert 已杜绝新增）：
    // 按 (month, update_time) 升序遍历、后者覆盖前者，为每组保留最新一条，避免重复累加。
    let mut latest_by_book_month: HashMap<(String, i32), MonthRecordModel> = HashMap::new();
    for record in records {
        latest_by_book_month.insert((record.book_id.clone(), record.month), record);
    }

    // 跨账簿按月合计：某月净资产 = 该月各在册账簿净资产之和（账簿无该月记录则不参与）。
    // BTreeMap 保证按月份升序，首个元素即「年初」、末个元素即「年末」。
    let mut monthly: BTreeMap<i32, (Decimal, Decimal, Decimal)> = BTreeMap::new();
    for record in latest_by_book_month.into_values() {
        let entry =
            monthly
                .entry(record.month)
                .or_insert((Decimal::ZERO, Decimal::ZERO, Decimal::ZERO));
        entry.0 += record.total_asset;
        entry.1 += record.total_liability;
        entry.2 += record.net_asset;
    }

    let first_agg = monthly
        .values()
        .next()
        .expect("records 非空时 monthly 必非空");
    let last_agg = monthly
        .values()
        .next_back()
        .expect("records 非空时 monthly 必非空");

    let item_count = ItemEntity::find()
        .filter(ItemCol::Year.eq(year))
        .filter(ItemCol::BookId.is_in(book_ids))
        .filter(ItemCol::IsDeleted.eq(0))
        .count(db)
        .await?;

    Ok(Some(YearSummary {
        year,
        book_count,
        item_count,
        recorded_months: monthly.len(),
        net_asset_start: first_agg.2,
        net_asset_end: last_agg.2,
        total_asset_end: last_agg.0,
        total_liability_end: last_agg.1,
    }))
}

/// 将年度总结组装为邮件模板参数（金额/百分比均为展示格式）。
fn build_yearly_mail_params(user: &UserModel, summary: YearSummary) -> MailParams {
    // 净资产变化金额：正数带「+」，负数自带「-」
    let change = summary.net_asset_end - summary.net_asset_start;
    let change_text = if change.is_sign_negative() {
        format_money(change)
    } else {
        format!("+{}", format_money(change))
    };
    // 年初净资产为零时无对比基准，变化率展示为「—」（calculate_growth_rate 分母为零返回 0.00）
    let rate_text = if summary.net_asset_start.is_zero() {
        "—".to_string()
    } else {
        format!(
            "{:+}%",
            calculate_growth_rate(summary.net_asset_end, summary.net_asset_start)
        )
    };

    let mut params = MailParams::new();
    params.to = user.email.clone();
    params.user_name = user.nickname.clone();
    params.user_email = user.email.clone();
    params.summary_year = summary.year.to_string();
    params.recorded_months = summary.recorded_months.to_string();
    params.book_count = summary.book_count.to_string();
    params.item_count = summary.item_count.to_string();
    params.total_asset = format_money(summary.total_asset_end);
    params.total_liability = format_money(summary.total_liability_end);
    params.net_asset = format_money(summary.net_asset_end);
    params.net_asset_start = format_money(summary.net_asset_start);
    params.net_asset_change = change_text;
    params.net_asset_change_rate = rate_text;
    params
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};

    /// 建好任务涉及的全部表（用户 / 账簿 / 汇总 / 明细 / 配置），返回内存库连接。
    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:?cache=shared")
            .await
            .expect("连接内存库失败");
        for ddl in [
            "CREATE TABLE IF NOT EXISTS SYS_USER (
                MR_ID TEXT PRIMARY KEY, MR_EMAIL TEXT, MR_PASSWORD TEXT, MR_NICKNAME TEXT,
                MR_ADMIN INTEGER DEFAULT 0, MR_STATUS INTEGER DEFAULT 0, MR_CANCEL_TIME TEXT,
                MR_REMIND_ENABLED INTEGER DEFAULT 0, MR_REMIND_DAY INTEGER,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_BOOK (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_NAME TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_ID TEXT,
                MR_YEAR INTEGER, MR_MONTH INTEGER,
                MR_TOTAL_ASSET REAL, MR_TOTAL_LIABILITY REAL, MR_NET_ASSET REAL,
                MR_MONTH_ON_MONTH REAL, MR_YEAR_ON_YEAR REAL, MR_NOTE TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_ITEM_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_YEAR INTEGER, MR_MONTH INTEGER, MR_BOOK_ID TEXT,
                MR_TEMPLATE_ITEM_ID TEXT, MR_ITEM_VALUE REAL,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            // SYS_CONFIG：站点地址与管理员邮箱，供邮件模板渲染占位符使用
            "CREATE TABLE IF NOT EXISTS SYS_CONFIG (
                MR_ID TEXT PRIMARY KEY, MR_CONFIG_KEY TEXT, MR_CONFIG_VALUE TEXT,
                MR_REMARK TEXT, MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }
        db.execute_unprepared(
            "INSERT INTO SYS_CONFIG (MR_ID, MR_CONFIG_KEY, MR_CONFIG_VALUE)
             VALUES ('cfg-site', 'webSite', 'https://mrecord.example.com'),
                    ('cfg-admin', 'adminMail', 'admin@mrecord.example.com')",
        )
        .await
        .expect("写入站点配置失败");
        db
    }

    /// 插入一位用户（默认开启邮件提醒、状态正常）。
    async fn insert_user(db: &DatabaseConnection, id: &str) {
        db.execute_unprepared(&format!(
            "INSERT INTO SYS_USER (MR_ID, MR_EMAIL, MR_PASSWORD, MR_NICKNAME, MR_STATUS, MR_REMIND_ENABLED)
             VALUES ('{id}', '{id}@test.com', 'hashed', '用户{id}', 0, 1)"
        ))
        .await
        .unwrap();
    }

    /// 为账簿写入一条月度汇总（年/月/总资产/总负债/净资产）。
    #[allow(clippy::too_many_arguments)]
    async fn insert_month_record(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
        book_id: &str,
        year: i32,
        month: i32,
        asset: i64,
        liability: i64,
    ) {
        db.execute_unprepared(&format!(
            "INSERT INTO FIN_MONTH_RECORD
                (MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH,
                 MR_TOTAL_ASSET, MR_TOTAL_LIABILITY, MR_NET_ASSET, MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR)
             VALUES ('{id}', '{user_id}', '{book_id}', {year}, {month},
                     {asset}, {liability}, {}, 0, 0)",
            asset - liability
        ))
        .await
        .unwrap();
    }

    async fn count_users(db: &DatabaseConnection) -> i64 {
        let row = db
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) AS c FROM SYS_USER".to_string(),
            ))
            .await
            .unwrap()
            .unwrap();
        row.try_get::<i64>("", "c").unwrap()
    }

    // ==================== build_year_summary 聚合逻辑 ====================

    fn user(id: &str) -> UserModel {
        UserModel {
            id: id.to_string(),
            email: format!("{id}@test.com"),
            password: String::new(),
            nickname: format!("用户{id}"),
            admin: 0,
            status: 0,
            cancel_time: None,
            remind_enabled: 1,
            remind_day: None,
            create_by: None,
            create_time: NaiveDateTime::default(),
            update_by: None,
            update_time: None,
            is_deleted: 0,
        }
    }

    /// 跨账簿按月合计：年初取首月、年末取末月，明细条数与账簿数正确。
    #[tokio::test]
    async fn aggregates_across_books_by_month() {
        let db = setup_db().await;
        db.execute_unprepared(
            "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES
                ('bk1', 'u1', '日常账簿'), ('bk2', 'u1', '理财账簿')",
        )
        .await
        .unwrap();
        // bk1：1 月净资产 100、12 月 200；bk2：12 月净资产 50
        insert_month_record(&db, "mr1", "u1", "bk1", 2025, 1, 100, 0).await;
        insert_month_record(&db, "mr2", "u1", "bk1", 2025, 12, 220, 20).await;
        insert_month_record(&db, "mr3", "u1", "bk2", 2025, 12, 60, 10).await;
        db.execute_unprepared(
            "INSERT INTO FIN_MONTH_ITEM_RECORD (MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID, MR_TEMPLATE_ITEM_ID, MR_ITEM_VALUE)
             VALUES ('it1', 2025, 1, 'bk1', 't1', 100), ('it2', 2025, 12, 'bk2', 't2', 60)",
        )
        .await
        .unwrap();

        let summary = build_year_summary(&db, &user("u1"), 2025)
            .await
            .expect("聚合失败")
            .expect("应有数据");

        assert_eq!(summary.year, 2025);
        assert_eq!(summary.book_count, 2);
        assert_eq!(summary.recorded_months, 2); // 1 月与 12 月
        assert_eq!(summary.item_count, 2);
        // 年初 = 1 月净资产合计 100；年末 = 12 月净资产合计 (200 + 50) = 250
        assert_eq!(summary.net_asset_start, Decimal::from(100));
        assert_eq!(summary.net_asset_end, Decimal::from(250));
        // 年末总资产/总负债 = (220 + 60) / (20 + 10)
        assert_eq!(summary.total_asset_end, Decimal::from(280));
        assert_eq!(summary.total_liability_end, Decimal::from(30));
    }

    /// 历史 Java 数据的重复行（同账簿同月多行）只取最新一条，不重复累加。
    #[tokio::test]
    async fn deduplicates_legacy_duplicate_rows() {
        let db = setup_db().await;
        db.execute_unprepared(
            "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES ('bk1', 'u1', '账簿')",
        )
        .await
        .unwrap();
        // 重复行：1 月两条（旧 100 / 新 150），旧记录 update_time 更早
        db.execute_unprepared(
            "INSERT INTO FIN_MONTH_RECORD
                (MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH,
                 MR_TOTAL_ASSET, MR_TOTAL_LIABILITY, MR_NET_ASSET,
                 MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR, MR_UPDATE_TIME)
             VALUES
                ('old', 'u1', 'bk1', 2025, 1, 100, 0, 100, 0, 0, '2025-01-05 10:00:00'),
                ('new', 'u1', 'bk1', 2025, 1, 160, 10, 150, 0, 0, '2025-01-06 10:00:00')",
        )
        .await
        .unwrap();

        let summary = build_year_summary(&db, &user("u1"), 2025)
            .await
            .expect("聚合失败")
            .expect("应有数据");

        // 只保留最新行：净资产 150，而非 100 + 150 = 250
        assert_eq!(summary.net_asset_start, Decimal::from(150));
        assert_eq!(summary.net_asset_end, Decimal::from(150));
        assert_eq!(summary.recorded_months, 1);
    }

    /// 用户该年度无任何月度汇总 → None（不发送空洞总结）。
    #[tokio::test]
    async fn returns_none_when_no_records_for_year() {
        let db = setup_db().await;
        db.execute_unprepared(
            "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES ('bk1', 'u1', '账簿')",
        )
        .await
        .unwrap();
        // 只有 2024 年的数据，总结 2025 年时应返回 None
        insert_month_record(&db, "mr1", "u1", "bk1", 2024, 12, 100, 0).await;

        assert!(
            build_year_summary(&db, &user("u1"), 2025)
                .await
                .expect("聚合失败")
                .is_none()
        );
    }

    /// 没有任何账簿 → None。
    #[tokio::test]
    async fn returns_none_when_user_has_no_books() {
        let db = setup_db().await;
        assert!(
            build_year_summary(&db, &user("nobody"), 2025)
                .await
                .expect("聚合失败")
                .is_none()
        );
    }

    // ==================== build_yearly_mail_params 展示格式 ====================

    #[test]
    fn mail_params_format_money_and_growth_rate() {
        let mut params = MailParams::new();
        params.user_name = "张三".to_string();
        params.summary_year = "2025".to_string();
        params.recorded_months = "10".to_string();
        params.book_count = "2".to_string();
        params.item_count = "96".to_string();
        params.total_asset = format_money(Decimal::from(1234567));
        params.total_liability = "120,000.00".to_string();
        params.net_asset = format_money(Decimal::new(111456789, 2));
        params.net_asset_start = "900,000.00".to_string();
        params.net_asset_change = "+214,567.89".to_string();
        params.net_asset_change_rate = "+23.84%".to_string();

        let map = params.to_placeholders(Some("https://mrecord.example.com"), None);
        assert_eq!(map.get("MR-SummaryYear").unwrap(), "2025");
        assert_eq!(map.get("MR-RecordedMonths").unwrap(), "10");
        assert_eq!(map.get("MR-BookCount").unwrap(), "2");
        assert_eq!(map.get("MR-ItemCount").unwrap(), "96");
        assert_eq!(map.get("MR-TotalAsset").unwrap(), "1,234,567.00");
        assert_eq!(map.get("MR-NetAsset").unwrap(), "1,114,567.89");
        assert_eq!(map.get("MR-AdminMail").unwrap(), "");
    }

    #[test]
    fn change_text_carries_sign_and_rate_handles_zero_base() {
        // 正变化：+金额 / +百分比
        let up = build_yearly_mail_params(
            &user("u"),
            YearSummary {
                year: 2025,
                book_count: 1,
                item_count: 1,
                recorded_months: 2,
                net_asset_start: Decimal::from(100),
                net_asset_end: Decimal::from(150),
                total_asset_end: Decimal::from(150),
                total_liability_end: Decimal::ZERO,
            },
        );
        assert_eq!(up.net_asset_change, "+50.00");
        assert_eq!(up.net_asset_change_rate, "+50.00%");

        // 负变化：-金额 / -百分比
        let down = build_yearly_mail_params(
            &user("u"),
            YearSummary {
                year: 2025,
                book_count: 1,
                item_count: 1,
                recorded_months: 2,
                net_asset_start: Decimal::from(200),
                net_asset_end: Decimal::from(120),
                total_asset_end: Decimal::from(120),
                total_liability_end: Decimal::ZERO,
            },
        );
        assert_eq!(down.net_asset_change, "-80.00");
        assert_eq!(down.net_asset_change_rate, "-40.00%");

        // 年初为零：变化金额带 +，变化率为「—」
        let from_zero = build_yearly_mail_params(
            &user("u"),
            YearSummary {
                year: 2025,
                book_count: 1,
                item_count: 1,
                recorded_months: 1,
                net_asset_start: Decimal::ZERO,
                net_asset_end: Decimal::from(88),
                total_asset_end: Decimal::from(88),
                total_liability_end: Decimal::ZERO,
            },
        );
        assert_eq!(from_zero.net_asset_change, "+88.00");
        assert_eq!(from_zero.net_asset_change_rate, "—");
    }

    // ==================== run_once 端到端（邮件配置缺失时优雅跳过）====================

    /// 开启提醒且有年度数据的用户被处理；无数据 / 关闭提醒 / 非正常状态的用户跳过。
    #[tokio::test]
    async fn run_once_processes_eligible_users_only() {
        let db = setup_db().await;
        // u1：开启提醒 + 2025 年有数据
        insert_user(&db, "u1").await;
        db.execute_unprepared(
            "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES ('bk1', 'u1', '账簿')",
        )
        .await
        .unwrap();
        insert_month_record(&db, "mr1", "u1", "bk1", 2025, 6, 100, 0).await;
        // u2：开启提醒但无 2025 年数据
        insert_user(&db, "u2").await;
        // u3：关闭邮件提醒
        insert_user(&db, "u3").await;
        db.execute_unprepared("UPDATE SYS_USER SET MR_REMIND_ENABLED = 0 WHERE MR_ID = 'u3'")
            .await
            .unwrap();
        // u4：已停用
        insert_user(&db, "u4").await;
        db.execute_unprepared("UPDATE SYS_USER SET MR_STATUS = 1 WHERE MR_ID = 'u4'")
            .await
            .unwrap();

        // 总结年份 = 当前年 - 1；把 u1 的数据放到该年份确保命中
        let summary_year = Local::now().year() - 1;
        if summary_year != 2025 {
            db.execute_unprepared(&format!(
                "UPDATE FIN_MONTH_RECORD SET MR_YEAR = {summary_year} WHERE MR_ID = 'mr1'"
            ))
            .await
            .unwrap();
        }

        let config_service = crate::service::sys_config::SysConfigService::new();
        let email_service = EmailService::new(config_service);
        let task = YearlySummaryTask::new(email_service);

        let sent = task.run_once(&db).await.expect("任务执行失败");

        // 仅 u1 被处理：测试库未配置 SMTP，邮件被优雅跳过（记为成功处理）
        assert_eq!(sent, 1);
        assert_eq!(count_users(&db).await, 4);
    }
}
