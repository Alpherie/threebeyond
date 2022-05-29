use super::*;
use serde::Serializer;

pub mod multipart;

pub fn serialize_naive<S>(t: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i64(t.timestamp())
}

pub fn serialize_naive_option<S>(t: &Option<NaiveDateTime>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match t {
        Some(v) => serialize_naive(v, s),
        None => s.serialize_unit(),
    }
}
