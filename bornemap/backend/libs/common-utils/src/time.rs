use chrono::Utc;

pub fn now_utc() -> chrono::DateTime<Utc> {
    Utc::now()
}

pub fn to_rfc3339(dt: &chrono::DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
