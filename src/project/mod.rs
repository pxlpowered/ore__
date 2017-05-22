/*
 Crate:         ore
 File:          /project/mod.rs
 Module:        ::project
 Visibility:    private
 */

// TODO: documentation

pub mod category;
pub mod channel;
pub mod member;
pub mod version;

use chrono::{DateTime, UTC};
use self::category::Category;
use self::channel::Channel;
use self::member::Member;
use self::version::Version;
use serialize::{deserialize_datetime, serialize_datetime};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

// // TODO: documentation
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project<'a> {
    category: Category,
    channels: Vec<Channel<'a>>,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    created_at: DateTime<UTC>,
    description: &'a str,
    downloads: u32,
    href: &'a str,
    members: Vec<Member<'a>>,
    name: &'a str,
    owner: &'a str,
    plugin_id: &'a str,
    recommended: Version<'a>,
    stars: u32,
    views: u32,
}
