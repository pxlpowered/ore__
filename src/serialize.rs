/*
 Crate:         ore
 File:          /serialize.rs
 Module:        ::serialize
 Visibility:    private
 */

// TODO: documentation

use chrono::{DateTime, TimeZone, UTC};
use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error as SerdeError;

const DATE_TIME_FMT: &'static str = "%F %T%.3f";

// TODO: documentation
pub fn serialize_datetime<S>(datetime: &DateTime<UTC>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_str(&format!("{}", datetime.format(DATE_TIME_FMT)))
}

// TODO: documentation
pub fn deserialize_datetime<'a, D>(deserializer: D) -> Result<DateTime<UTC>, D::Error>
    where D: Deserializer<'a>
{
    let de = String::deserialize(deserializer)?;
    UTC.datetime_from_str(&de, DATE_TIME_FMT)
        .map_err(SerdeError::custom)
}
