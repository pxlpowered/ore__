/*
 Crate:         ore
 File:          /project/version.rs
 Module:        ::project::version
 Visibility:    public
 */

// TODO: documentation

use chrono::{DateTime, UTC};
use project::channel::Channel;
use serialize::{deserialize_datetime, serialize_datetime};
use std::fmt::{Display, Formatter, Result as FmtResult};

// TODO: documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version<'a> {
    channel: Channel<'a>,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    created_at: DateTime<UTC>,
    dependencies: Vec<Dependency<'a>>,
    file_size: u32,
    id: u32,
    name: &'a str,
    plugin_id: &'a str,
    staff_approved: bool,
}

impl<'a> Version<'a> {

    // TODO: documentation
    pub fn channel(&self) -> Channel
    {
        self.channel.to_owned()
    }

    // TODO: documentation
    pub fn created_at(&self) -> String {
        use serialize::DATE_TIME_FMT;
        format!("{}", self.created_at.format(DATE_TIME_FMT))
    }

    // TODO: documentation
    pub fn dependencies(&self) -> Vec<Dependency> {
        self.dependencies.to_vec()
    }

    // TODO: documentation
    pub fn file_size(&self) -> u32 {
        self.file_size
    }

    // TODO: documentation
    pub fn id(&self) -> u32 {
        self.id
    }

    // TODO: documentation
    pub fn name(&self) -> &str {
        self.name
    }

    // TODO: documentation
    pub fn plugin_id(&self) -> &str {
        self.plugin_id
    }

    // TODO: documentation
    pub fn staff_approved(&self) -> bool {
        self.staff_approved
    }
}

// TODO: documentation
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency<'a> {
    plugin_id: &'a str,
    version: &'a str,
}

impl<'a> Dependency<'a> {

    // TODO: documentation
    pub fn plugin_name(&self) -> &str {
        self.plugin_id
    }

    // TODO: documentation
    pub fn version(&self) -> &str {
        self.version
    }
}

impl<'a> Display for Dependency<'a> {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}@{}", self.plugin_id, self.version)
    }
}
