use serde::{Deserialize, Deserializer};

/// When deserializing and the content is a zero, it returns [`Option::None`] otherwise
/// [`Option::Some`] with the value that was deserialized.
pub(crate) fn deserialize_zero_to_none<'de, D, T: Deserialize<'de> + num_traits::Zero>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Value<U>(Option<U>);
    let v: Value<T> = Deserialize::deserialize(deserializer)?;
    let result = match v.0 {
        Some(v) => {
            if v.is_zero() {
                None
            } else {
                Some(v)
            }
        }
        None => None,
    };
    Ok(result)
}
