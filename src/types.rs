use serde::de::{Visitor, Deserializer, Deserialize, Error as SerdeError, MapAccess};
use serde::ser::{Serializer, Serialize};
use std::fmt::{Formatter, Result as FmtResult};
use std::error::Error;
use chrono::{DateTime, UTC, TimeZone};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum SortType {
    MostStars = 0,
    MostDownloads = 1,
    MostViews = 2,
    Newest = 3,
    RecentlyUpdated = 4,
    Undefined,
}

struct SortTypeVisitor;

impl<'a> Visitor<'a> for SortTypeVisitor {
    type Value = SortType;
    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "an integer between 0 and 4")
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where E: Error
    {
        let res = match v {
            0 => SortType::MostStars,
            1 => SortType::MostDownloads,
            2 => SortType::MostViews,
            3 => SortType::Newest,
            4 => SortType::RecentlyUpdated,
            _ => SortType::Undefined,
        };
        Ok(res)
    }
}

impl Serialize for SortType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'a> Deserialize<'a> for SortType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_u8(SortTypeVisitor)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub created_at: DateTime<UTC>,
    pub username: String,
    pub roles: Vec<String>,
    pub starred: Vec<String>,
    pub avatar_template: String,
    pub avatar_url: String,
    // pub projects: Vec<Plugin>,
    #[serde(skip_serializing, skip_deserializing)]
    lock: (),
}

const DATETIME_FMT: &'static str = "%F %T%.3f";
fn serialize_datetime<S>(datetime: &DateTime<UTC>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_str(&format!("{}", datetime.format(DATETIME_FMT)))
}

fn deserialize_datetime<'a, D>(deserializer: D) -> Result<DateTime<UTC>, D::Error>
    where D: Deserializer<'a>
{
    let str = String::deserialize(deserializer)?;
    UTC.datetime_from_str(&str, DATETIME_FMT)
        .map_err(SerdeError::custom)
}
