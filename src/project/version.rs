/*
 Crate:         ore
 File:          /project/version.rs
 Module:        ::project::version
 Visibility:    public
 */

// TODO: documentation

use chrono::{DateTime, UTC};
use project::channel::Channel;
use request::{Error as RequestError, Request};
use serialize::{deserialize_datetime, serialize_datetime};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::Project;
use super::get_project;

const VERSIONS: &'static str = "/versions";

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
    pub fn channel(&self) -> Channel {
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
    pub fn project_name(&self) -> String {
        self.plugin_id.to_owned()
    }

    // TODO: documentation
    pub fn version(&self) -> String {
        self.version.to_owned()
    }

    // TODO: documentation
    pub fn get_project(&self, url: &str) -> Result<Project, RequestError> {
        super::get_project(self.plugin_id.as_str(), url)
    }
}

impl Display for Dependency {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}@{}", self.plugin_id, self.version)
    }
}

#[derive(Clone, Debug)]
pub struct VersionsRequest {
    channels: Option<Vec<Channel>>,
    limit: Option<u32>,
    offset: Option<u32>,
    _project: Box<Project>,
}

impl VersionsRequest {

    // TODO: documentation
    #[inline]
    pub fn new_from_id(project_id: String, url: &str) -> Result<Self, RequestError> {
        Self::new_from_project(&get_project(project_id.as_str(), url)?)
    }

    // TODO: documentation
    pub fn new_from_project(project: &Project) -> Result<Self, RequestError> {
        Ok(VersionsRequest {
            channels: None,
            limit: None,
            offset: None,
            _project: Box::new(project.to_owned())
        })
    }

    // TODO: documentation
    pub fn channels(&self) -> Option<Vec<Channel>> {
        self.channels.to_owned()
    }

    // TODO: documentation
    pub fn add_channel(&mut self, channel: &Channel) -> &mut Self {

        match self.channels {
            Some(ref mut channels) => channels.push(channel.to_owned()),
            None => self.channels = Some(vec![channel.to_owned()]),
        }

        self
    }

    // TODO: documentation
    pub fn set_channels(&mut self, channels: &Vec<Channel>) -> &mut Self {
        self.channels = Some(channels.to_vec());
        self
    }

    // TODO: documentation
    pub fn reset_channels(&mut self) -> &mut Self {
        self.channels = None;
        self
    }

    // TODO: documentation
    pub fn limit(&self) -> Option<u32> {
        self.limit
    }

    // TODO: documentation
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    // TODO: documentation
    pub fn reset_limit(&mut self) -> &mut Self {
        self.limit = None;
        self
    }

    // TODO: documentation
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }

    // TODO: documentation
    pub fn set_offset(&mut self, offset: u32) -> &mut Self {
        self.limit = Some(offset);
        self
    }

    // TODO: documentation
    pub fn reset_offset(&mut self) -> &mut Self {
        self.offset = None;
        self
    }
}

impl<'a> Request<'a> for VersionsRequest {
    type Ret = Vec<Version>;

    fn request(&self, url: &str) -> Result<Self::Ret, RequestError> {
        use hyper::{Client, Url};
        use hyper::net::HttpsConnector;
        use hyper_rustls::TlsClient;
        use serde_json;
        use std::io::Read;

        // /projects/:pluginId/versions
        let mut req_url = Url::parse((url.to_string() + self._project.project_id().as_str() + VERSIONS)
                                         .as_str())?;

        {
            let mut query_pairs = req_url.query_pairs_mut();

            if let Some(ref channels) = self.channels {
                query_pairs.append_pair("channels",
                                        channels
                                            .into_iter()
                                            .map(|c| c.name())
                                            .fold(String::new(), |o, n| o + "," + n.as_str())
                                            .as_str());
            }

            if let Some(ref limit) = self.limit {
                query_pairs.append_pair("limit", limit.to_string().as_str());
            }

            if let Some(ref offset) = self.offset {
                query_pairs.append_pair("offset", offset.to_string().as_str());
            }
        }

        let mut res = String::new();
        Client::with_connector(HttpsConnector::new(TlsClient::new()))
            .get(req_url)
            .send()?
            .read_to_string(&mut res)?;
        Ok(serde_json::from_str::<Self::Ret>(res.as_str())?)
    }
}
