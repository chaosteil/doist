use serde::Serializer;

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
