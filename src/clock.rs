//! Shared timestamp presentation in the OS timezone; wire epochs stay unchanged.
use chrono::{DateTime, Datelike, Local, TimeZone};

pub fn normalize_ts_ms(ts: i64) -> i64 {
    if ts <= 0 {
        0
    } else if ts < 10_000_000_000 {
        ts.saturating_mul(1000)
    } else {
        ts
    }
}

pub fn local_datetime(ts: i64) -> Option<DateTime<Local>> {
    let ms = normalize_ts_ms(ts);
    (ms > 0)
        .then(|| Local.timestamp_millis_opt(ms).single())
        .flatten()
}

pub fn message_time(ts: i64) -> String {
    message_time_in(ts, &Local)
}

fn message_time_in<T: TimeZone>(ts: i64, timezone: &T) -> String
where
    T::Offset: std::fmt::Display,
{
    let ms = normalize_ts_ms(ts);
    (ms > 0)
        .then(|| timezone.timestamp_millis_opt(ms).single())
        .flatten()
        .map(|date| date.format("%H:%M").to_string())
        .unwrap_or_default()
}

/// Sidebar timestamps switch at an elapsed 24 hours, not at local midnight.
pub fn sidebar_time_at(ts: i64, now: i64) -> String {
    sidebar_time_in(ts, now, &Local)
}

pub fn sidebar_time_in<T: TimeZone>(ts: i64, now: i64, timezone: &T) -> String
where
    T::Offset: std::fmt::Display,
{
    let ts = normalize_ts_ms(ts);
    let now = normalize_ts_ms(now);
    let Some(date) = timezone
        .timestamp_millis_opt(ts)
        .single()
        .filter(|_| ts > 0)
    else {
        return String::new();
    };
    if now.saturating_sub(ts) < 86_400_000 {
        return date.format("%H:%M").to_string();
    }
    let same_year = timezone
        .timestamp_millis_opt(now)
        .single()
        .is_some_and(|current| current.year() == date.year());
    date.format(if same_year { "%d/%m" } else { "%d/%m/%Y" })
        .to_string()
}

pub fn local_day(ts: i64) -> Option<i64> {
    local_datetime(ts).map(|dt| dt.date_naive().num_days_from_ce() as i64)
}

pub fn now_clock() -> String {
    Local::now().format("%H:%M:%S").to_string()
}
