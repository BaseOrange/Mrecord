//! 月度记账提醒定时任务
//!
//! 对应 Java: `com.dcz.mrecord.task.MonthlyReminderTask`
//!
//! Java 端使用 `@Scheduled(cron = "0 8 8 * * ?")` 每天 08:08 执行。Rust 端通过 Tokio 后台任务
//! 计算下一次 08:08 的等待时长，并在触发时查询需要提醒的用户后发送邮件。

use std::sync::Arc;

use chrono::{Datelike, Local, NaiveDate};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    constant::user_status::UserStatus,
    entity::sys_user::{Column as UserCol, Entity as UserEntity, Model as UserModel},
    error::AppError,
    model::mail_params::MailParams,
    service::email::EmailService,
    util::schedule::duration_until_daily,
};

/// 月度记账提醒任务服务。
///
/// 负责迁移 Java `MonthlyReminderTask.monthlyReminder` 的用户筛选和邮件发送逻辑。
pub struct MonthlyReminderTask {
    email_service: Arc<EmailService>,
}

impl MonthlyReminderTask {
    /// 创建月度提醒任务服务实例。
    pub fn new(email_service: Arc<EmailService>) -> Arc<Self> {
        Arc::new(Self { email_service })
    }

    /// 启动后台定时循环。
    ///
    /// 对应 Java 应用启动后由 `@EnableScheduling` 自动注册的 `MonthlyReminderTask`。
    pub fn start(self: Arc<Self>, db: DatabaseConnection) {
        tokio::spawn(async move {
            loop {
                let wait = duration_until_daily(8, 8);
                tracing::info!("月度记账提醒任务将在 {} 秒后执行", wait.as_secs());
                tokio::time::sleep(wait).await;

                if let Err(e) = self.run_once(&db).await {
                    tracing::error!("月度记账提醒任务执行失败: {:?}", e);
                }
            }
        });
    }

    /// 执行一次月度记账提醒。
    ///
    /// 对应 Java: `MonthlyReminderTask.monthlyReminder`。
    pub async fn run_once(&self, db: &DatabaseConnection) -> Result<(), AppError> {
        let today = Local::now().date_naive();
        let users = load_remind_users(db, today).await?;
        tracing::info!("月度记账提醒任务匹配到 {} 位用户", users.len());

        for user in users {
            let params = build_monthly_mail_params(&user);
            if let Err(e) = self.email_service.send_month_report_email(db, params).await {
                tracing::error!(user_id = %user.id, email = %user.email, "月度记账提醒邮件发送失败: {:?}", e);
            }
        }

        Ok(())
    }
}

/// 查询当天需要发送月度记账提醒的用户。
///
/// 对应 Java 中 `remindDay == today`，月末额外包含 `remindDay > today` 的补偿查询。
async fn load_remind_users(
    db: &DatabaseConnection,
    today: NaiveDate,
) -> Result<Vec<UserModel>, AppError> {
    let day = today.day() as i32;
    let mut condition = Condition::any().add(UserCol::RemindDay.eq(day));
    if is_last_day_of_month(today) {
        condition = condition.add(UserCol::RemindDay.gt(day));
    }

    Ok(UserEntity::find()
        .filter(UserCol::RemindEnabled.eq(1))
        .filter(UserCol::Status.eq(UserStatus::Normal as i32))
        .filter(UserCol::IsDeleted.eq(0))
        .filter(condition)
        .all(db)
        .await?)
}

/// 组装月度记账提醒邮件模板参数。
///
/// 对应 Java 中 `MailParamsBO` 的用户昵称、邮箱和当前年月占位符。
fn build_monthly_mail_params(user: &UserModel) -> MailParams {
    let mut params = MailParams::new();
    params.to = user.email.clone();
    params.user_name = user.nickname.clone();
    params.user_email = user.email.clone();
    params
}

/// 判断给定日期是否为所在月份最后一天。
fn is_last_day_of_month(date: NaiveDate) -> bool {
    date.succ_opt()
        .is_some_and(|next_day| next_day.month() != date.month())
}
