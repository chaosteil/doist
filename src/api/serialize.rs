use chrono::NaiveDate;
use serde::{Deserialize, Serializer, de};

/// This function is there to serialize the datetime into something that the Todoist API can
/// understand, as it doesn't quite implement the full rfc3339 spec and breaks with the default
/// chrono formatter.
pub(crate) fn todoist_rfc3339<S>(
    dt: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    serializer.serialize_str(&dt)
}

/// Deserializes due-date fields returned by the Todoist API.
///
/// The API can return either `YYYY-MM-DD` or a datetime string such as
/// `YYYY-MM-DDTHH:MM:SS`/RFC3339 for recurring tasks.
pub(crate) fn todoist_due_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;

    if let Ok(date) = NaiveDate::parse_from_str(&raw, "%Y-%m-%d") {
        return Ok(date);
    }

    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&raw, "%Y-%m-%dT%H:%M:%S") {
        return Ok(dt.date());
    }

    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&raw) {
        return Ok(dt.date_naive());
    }

    Err(de::Error::custom(format!(
        "invalid due.date format: {raw}"
    )))
}
