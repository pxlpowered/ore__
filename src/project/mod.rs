/*
 Crate:         ore
 File:          /project/mod.rs
 Module:        ::project
 Visibility:    public
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

impl<'a> Project<'a> {

    // TODO: documentation
    pub fn category(&self) -> Category {
        self.category
    }

    // TODO: documentation
    pub fn channels(&self) -> Vec<Channel> {
        self.channels.to_vec()
    }

    // TODO: documentation
    pub fn created_at(&self) -> String {
        use serialize::DATE_TIME_FMT;
        format!("{}", self.created_at.format(DATE_TIME_FMT))
    }

    // TODO: documentation
    pub fn description(&self) -> &str {
        self.description
    }

    // TODO: documentation
    pub fn downloads(&self) -> u32 {
        self.downloads
    }

    // TODO: documentation
    pub fn href(&self) -> &str {
        self.href
    }

    // TODO: documentation
    pub fn members(&self) -> Vec<Member> {
        self.members.to_vec()
    }

    // TODO: documentation
    pub fn name(&self) -> &str {
        self.name
    }

    // TODO: documentation
    pub fn owner(&self) -> &str {
        self.owner
    }

    // TODO: documentation
    pub fn plugin_id(&self) -> &str {
        self.plugin_id
    }

    // TODO: documentation
    pub fn recommended(&self) -> Version {
        self.recommended.to_owned()
    }

    // TODO: documentation
    pub fn stars(&self) -> u32 {
        self.stars
    }

    // TODO: documentation
    pub fn views(&self) -> u32 {
        self.views
    }
}
