//! 定时任务调度工具
//!
//! Java 端依赖 Spring 的 `@Scheduled(cron = ...)`，由框架的 `TaskScheduler` 自动触发；
//! Rust 端没有调度框架，定时任务（月度提醒、注销清理）需自行计算下一次触发时间并
//! `tokio::time::sleep`。本模块统一提供「每天本地时间固定时刻触发一次」的等待时长计算，
//! 供所有日级定时任务复用，等价于 Java 的 `@Scheduled(cron = "0 m H * * ?")`。

use std::time::Duration;

use chrono::{Datelike, Local, NaiveDate, NaiveTime, TimeZone};

/// 计算距离下一次每天 `hour:minute`（本地时间）触发的等待时长。
///
/// 若今天该时刻尚未到达，则等到今天；否则等到明天同一时刻。
/// 用于替代 Java `@Scheduled(cron = "0 {minute} {hour} * * ?")`。
///
/// # Panics
/// 仅当传入非法时分（如 25:61）时 panic，调用方均使用编译期常量。
pub fn duration_until_daily(hour: u32, minute: u32) -> Duration {
    let now = Local::now();
    let run_time = NaiveTime::from_hms_opt(hour, minute, 0).expect("固定时间合法");
    let today_run = Local
        .from_local_datetime(&now.date_naive().and_time(run_time))
        .single();

    let next_run = match today_run.filter(|t| *t > now) {
        Some(t) => t,
        None => {
            let tomorrow = now.date_naive().succ_opt().expect("日期递增合法");
            Local
                .from_local_datetime(&tomorrow.and_time(run_time))
                .single()
                .unwrap_or_else(|| now + chrono::Duration::days(1))
        }
    };

    (next_run - now)
        .to_std()
        .unwrap_or_else(|_| Duration::from_secs(60))
}

/// 计算距离下一次每年 `month` 月 `day` 日 `hour:minute`（本地时间）触发的等待时长。
///
/// 若今年该日期该时刻尚未到达，则等到今年；否则等到明年同一日期时刻。
/// 用于年度级定时任务，等价于 Java `@Scheduled(cron = "0 {minute} {hour} {day} {month} ?")`。
///
/// # Panics
/// 仅当传入非法月日/时分（如 13 月 32 日 25:61）时 panic，调用方均使用编译期常量。
pub fn duration_until_yearly(month: u32, day: u32, hour: u32, minute: u32) -> Duration {
    let now = Local::now();
    let run_time = NaiveTime::from_hms_opt(hour, minute, 0).expect("固定时间合法");

    let this_year_date = NaiveDate::from_ymd_opt(now.year(), month, day).expect("固定日期合法");
    let today_run = Local
        .from_local_datetime(&this_year_date.and_time(run_time))
        .single();

    let next_run = match today_run.filter(|t| *t > now) {
        Some(t) => t,
        None => {
            let next_date =
                NaiveDate::from_ymd_opt(now.year() + 1, month, day).expect("固定日期合法");
            Local
                .from_local_datetime(&next_date.and_time(run_time))
                .single()
                .unwrap_or_else(|| now + chrono::Duration::days(365))
        }
    };

    (next_run - now)
        .to_std()
        .unwrap_or_else(|_| Duration::from_secs(60))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_until_daily_within_one_day() {
        // 无论当前时间如何，下一次 08:08 必然在 24 小时之内
        let wait = duration_until_daily(8, 8);
        assert!(wait.as_secs() > 0);
        assert!(wait.as_secs() <= 24 * 60 * 60);
    }

    #[test]
    fn duration_until_daily_reaches_past_time_next_day() {
        // 已过去的时刻（凌晨 00:00）必然排在明天，等待时长接近一整天
        let now = Local::now();
        let wait = duration_until_daily(0, 0);
        // 与「明天 00:00 - 现在」的差值误差不超过 5 秒（计算与断言之间的时间漂移）
        let tomorrow_midnight = now
            .date_naive()
            .succ_opt()
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let expected = (Local
            .from_local_datetime(&tomorrow_midnight)
            .single()
            .unwrap()
            - now)
            .to_std()
            .unwrap();
        assert!((wait.as_secs() as i64 - expected.as_secs() as i64).abs() <= 5);
    }

    // ==================== duration_until_yearly 测试 ====================

    #[test]
    fn duration_until_yearly_within_one_year() {
        // 无论当前时间如何，下一次 1 月 1 日 08:08 必然在一年之内
        let wait = duration_until_yearly(1, 1, 8, 8);
        assert!(wait.as_secs() > 0);
        // 闰年最多 366 天
        assert!(wait.as_secs() <= 366 * 24 * 60 * 60);
    }

    #[test]
    fn duration_until_yearly_matches_expected_next_run() {
        // 独立重算期望的下一次触发时刻：今天 08:08 未过则今天，否则明年 1 月 1 日 08:08
        let now = Local::now();
        let run_time = NaiveTime::from_hms_opt(8, 8, 0).unwrap();
        let today_run = Local
            .from_local_datetime(&now.date_naive().and_time(run_time))
            .single()
            .unwrap();
        let expected = if today_run > now {
            today_run
        } else {
            let next_jan1 = NaiveDate::from_ymd_opt(now.year() + 1, 1, 1).unwrap();
            Local
                .from_local_datetime(&next_jan1.and_time(run_time))
                .single()
                .unwrap()
        };
        let wait = duration_until_yearly(1, 1, 8, 8);
        let expected_dur = (expected - now).to_std().unwrap();
        // 与「期望时刻 - 现在」的差值误差不超过 5 秒（计算与断言之间的时间漂移）
        assert!((wait.as_secs() as i64 - expected_dur.as_secs() as i64).abs() <= 5);
    }
}
