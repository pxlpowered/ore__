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
pub struct Version {
    channel: Channel,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    created_at: DateTime<UTC>,
    dependencies: Vec<Dependency>,
    file_size: u32,
    id: u32,
    name: String,
    plugin_id: String,
    staff_approved: bool,
}

impl Version {

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
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    // TODO: documentation
    pub fn plugin_id(&self) -> String {
        self.plugin_id.to_owned()
    }

    // TODO: documentation
    pub fn staff_approved(&self) -> bool {
        self.staff_approved
    }
}

// TODO: documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    plugin_id: String,
    version: String,
}

impl Dependency {

    // TODO: documentation
    pub fn plugin_name(&self) -> String {
        self.plugin_id.to_owned()
    }

    // TODO: documentation
    pub fn version(&self) -> String {
        self.version.to_owned()
    }
}

impl Display for Dependency {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}@{}", self.plugin_id, self.version)
    }
}
