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

// TODO: documentation
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency<'a> {
    plugin_id: &'a str,
    version: &'a str,
}

impl<'a> Display for Dependency<'a> {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}@{}", self.plugin_id, self.version)
    }
}
