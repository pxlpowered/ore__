use serde::de::{Visitor, Deserializer, Deserialize, Error as SerdeError, MapAccess};
use serde::ser::{Serializer, Serialize};
use std::fmt::{Formatter, Result as FmtResult};
use std::error::Error;
use chrono::{DateTime, UTC, TimeZone};
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum PluginCategory {
    AdminTools = 0,
    Chat = 1,
    DeveloperTools = 2,
    Economy = 3,
    Gameplay = 4,
    Games = 5,
    Protection = 6,
    RolePlaying = 7,
    WorldManagement = 8,
    Miscellaneous = 9,
    Undefined,
}

struct PluginCategoryVisitor;

impl<'a> Visitor<'a> for PluginCategoryVisitor {
    type Value = PluginCategory;
    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, r#"an integer between 0 and 9 or an object with a string value "category""#)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where E: Error {
        let res = match v {
            0 => PluginCategory::AdminTools,
            1 => PluginCategory::Chat,
            2 => PluginCategory::DeveloperTools,
            3 => PluginCategory::Economy,
            4 => PluginCategory::Gameplay,
            5 => PluginCategory::Games,
            6 => PluginCategory::Protection,
            7 => PluginCategory::RolePlaying,
            8 => PluginCategory::WorldManagement,
            9 => PluginCategory::Miscellaneous,
            _ => PluginCategory::Undefined,
        };
        Ok(res)
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where A: MapAccess<'a> {
        let mut category: Option<PluginCategory> = None;
        while let Ok(Some((key, value))) = map.next_entry::<String, String>() {
            if key == "title" {
                category = Some(match value.as_str() {
                    "Admin Tools" => PluginCategory::AdminTools,
                    "Chat" => PluginCategory::Chat,
                    "Developer Tools" => PluginCategory::DeveloperTools,
                    "Economy" => PluginCategory::Economy,
                    "Gameplay" => PluginCategory::Gameplay,
                    "Games" => PluginCategory::Games,
                    "Protection" => PluginCategory::Protection,
                    "Role Playing" => PluginCategory::RolePlaying,
                    "World Management" => PluginCategory::WorldManagement,
                    "Miscellaneous" => PluginCategory::Miscellaneous,
                    _ => PluginCategory::Undefined,
                });

            }
        }
        category.ok_or(SerdeError::custom(r#"Invalid value "category""#))
    }

}

impl Serialize for PluginCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'a> Deserialize<'a> for PluginCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'a> {
        deserializer.deserialize_any(PluginCategoryVisitor)
    }
}

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
        where E: Error {
        let res = match v {
            0 => SortType::MostStars,
            1 => SortType::MostDownloads,
            2 => SortType::MostViews,
            3 => SortType::Newest,
            4 => SortType::RecentlyUpdated,
            _ => SortType::Undefined
        };
        Ok(res)
    }
}

impl Serialize for SortType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'a> Deserialize<'a> for SortType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'a> {
        deserializer.deserialize_u8(SortTypeVisitor)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub plugin_id: String,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub created_at: DateTime<UTC>,
    pub name: String,
    pub owner: String,
    pub description: String,
    pub href: String,
    pub members: Vec<ShortUser>,
    pub channels: Vec<Channel>,
    pub recommended: Version,
    pub category: PluginCategory,
    pub views: u32,
    pub downloads: u32,
    pub stars: u32,
    #[serde(skip_serializing, skip_deserializing)]
    lock: (),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortUser {
    pub user_id: u32,
    pub name: String,
    pub roles: Vec<String>,
    pub head_role: String,
    #[serde(skip_serializing, skip_deserializing)]
    lock: (),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub name: String,
    pub color: String,
    #[serde(skip_serializing, skip_deserializing)]
    lock: (),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: u32,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub created_at: DateTime<UTC>,
    pub name: String,
    pub dependencies: Vec<Dependency>,
    pub plugin_id: String,
    pub channel: Channel,
    pub file_size: u32,
    pub staff_approved: bool,
    #[serde(skip_serializing, skip_deserializing)]
    lock: (),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub plugin_id: String,
    pub version: String,
    #[serde(skip_serializing, skip_deserializing)]
    lock: (),
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
    pub projects: Vec<Plugin>,
    #[serde(skip_serializing, skip_deserializing)]
    lock: (),
}

const DATETIME_FMT: &'static str = "%F %T%.3f";
fn serialize_datetime<S>(datetime: &DateTime<UTC>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    serializer.serialize_str(&format!("{}", datetime.format(DATETIME_FMT)))
}

fn deserialize_datetime<'a, D>(deserializer: D) -> Result<DateTime<UTC>, D::Error>
    where D: Deserializer<'a> {
    let str = String::deserialize(deserializer)?;
    UTC.datetime_from_str(&str, DATETIME_FMT).map_err(SerdeError::custom)
}
