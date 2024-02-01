use serde::Deserialize;

pub mod v4;

/// bool to int serialize function
pub fn bool_to_int<S>(b: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match b {
        Some(true) => s.serialize_u32(1),
        Some(false) => s.serialize_u32(0),
        None => s.serialize_none(),
    }
}

/// int to bool deserialize function
pub fn int_to_bool<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i = u32::deserialize(d)?;
    match i {
        1 => Ok(Some(true)),
        0 => Ok(Some(false)),
        _ => Ok(None),
    }
}