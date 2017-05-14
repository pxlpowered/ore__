/*
 Crate:         ore
 File:          /project/mod.rs
 Module:        ::project
 Visibility:    private
 */

// TODO: documentation

use chrono::{DateTime, UTC};
use serialize::{deserialize_datetime, serialize_datetime};

// TODO: documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel<'a> {
    color: &'a str,
    name: &'a str,
}

// TODO: documentation
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency<'a> {
    plugin_id: &'a str,
    version: &'a str,
}

// TODO: documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member<'a> {
    head_role: Role,
    name: &'a str,
    roles: Vec<Role>,
    user_id: u32,
}

// TODO: documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project<'a> {
    category: Category,
    channels: Vec<Channel<'a>>,
    #[serde(serialize_with = "serialize_datetime", deserialize_with =
    "deserialize_datetime")]
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

// TODO: documentation
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Category {
    AdminTools = 0,
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

// TODO: documentation
#[repr(u8)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Role {
    Admin,
    Developer,
    Editor,
    Owner,
    Support,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version<'a> {
    channel: Channel<'a>,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    created_at: DateTime<UTC>,
    file_size: u32,
    dependencies: Vec<Dependency<'a>>,
    name: &'a str,
    staff_approved: bool,
}

mod category_impl;
