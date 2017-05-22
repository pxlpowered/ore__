/*
 Crate:         ore
 File:          /project/category.rs
 Module:        ::project
 Visibility:    public
 */

// TODO: documentation

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error as DeserializeError, MapAccess, Visitor};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

// TODO: documentation
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Category {
    AdminTools,
    Chat,
    DeveloperTools,
    Economy,
    Gameplay,
    Games,
    Protection,
    RolePlaying,
    WorldManagement,
    Miscellaneous,
    Undefined,
}

impl Default for Category {
    fn default() -> Self {
        Category::Undefined
    }
}

#[doc(hidden)]
impl<'a> Deserialize<'a> for Category {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_any(CategoryVisitor)
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
               "{}",
               match *self {
                   Category::AdminTools => "Admin Tools",
                   Category::Chat => "Chat",
                   Category::DeveloperTools => "Developer Tools",
                   Category::Economy => "Economy",
                   Category::Gameplay => "Gameplay",
                   Category::Games => "Games",
                   Category::Protection => "Protection",
                   Category::RolePlaying => "Role Playing",
                   Category::WorldManagement => "World Management",
                   Category::Miscellaneous => "Miscellaneous",
                   Category::Undefined => "Undefined",
               })
    }
}

#[doc(hidden)]
impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(*self as u64)
    }
}

struct CategoryVisitor;

impl<'a> Visitor<'a> for CategoryVisitor {
    type Value = Category;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
               r#"an integer between 0 and 9 or an object with a string value "category""#)
    }

    fn visit_map<A>(self, mut map: A) -> StdResult<Self::Value, A::Error>
        where A: MapAccess<'a>
    {
        let mut category = None;

        while let Ok(Some((key, value))) = map.next_entry::<&str, &str>() {
            if key == "title" {
                category = Some(match value {
                                    "Admin Tools" => Category::AdminTools,
                                    "Chat" => Category::Chat,
                                    "Developer Tools" => Category::DeveloperTools,
                                    "Economy" => Category::Economy,
                                    "Gameplay" => Category::Gameplay,
                                    "Games" => Category::Games,
                                    "Protection" => Category::Protection,
                                    "Role Playing" => Category::RolePlaying,
                                    "World Management" => Category::WorldManagement,
                                    "Miscellaneous" => Category::Miscellaneous,
                                    _ => Category::Undefined,
                                })
            }
        }

        category.ok_or(DeserializeError::custom(r#"invalid value "category""#))
    }

    fn visit_u8<E>(self, v: u8) -> StdResult<Self::Value, E>
        where E: DeserializeError
    {
        Ok(match v {
               0 => Category::AdminTools,
               1 => Category::Chat,
               2 => Category::DeveloperTools,
               3 => Category::Economy,
               4 => Category::Gameplay,
               5 => Category::Games,
               6 => Category::Protection,
               7 => Category::RolePlaying,
               8 => Category::WorldManagement,
               9 => Category::Miscellaneous,
               _ => Category::default(),
           })
    }

    fn visit_u64<E>(self, v: u64) -> StdResult<Self::Value, E>
        where E: DeserializeError
    {
        Self::visit_u8(self, v as u8)
    }
}
