/*
 Crate:         ore
 File:          /project/category_impl.rs
 Module:        ::project::category_impl
 Visibility:    private
 */

// TODO: documentation

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error as SerdeError, MapAccess, Visitor};
use std::error::Error as StdError;
use std::fmt::{Formatter, Result as FmtResult};
use super::{Category};

struct CategoryVisitor;

impl<'a> Visitor<'a> for CategoryVisitor {
    type Value = Category;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter,
               r#"an integer between 0 and 10 or an object with a string value "category""#)
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
        where E: StdError
    {
        Ok(match value {
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
            _ => Category::Undefined,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where A: MapAccess<'a>
    {
        let mut category = None;

        while let Ok(Some((key, value))) = map.next_entry::<String, String>() {
            if key == "title" {
                category = Some(match value.as_str() {
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
                });
            }
        }
        category.ok_or(SerdeError::custom(r#"Invalid value "category""#))
    }
}

#[doc(hidden)]
impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[doc(hidden)]
impl<'a> Deserialize<'a> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_any(CategoryVisitor)
    }
}