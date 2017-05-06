/*
 Crate:         ore
 File:          /lib.rs
 Module:        ::
 Visibility:    public
 */

extern crate chrono;
extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate hyper_rustls;

use serde::ser::{Serialize, Serializer};
use serde::de::{Visitor, Deserialize, Deserializer, Error as DeError, MapAccess};
use hyper::{Client, Url};
use hyper::error::Error as HyperError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt::{Formatter, Result as FmtResult, Display};
use chrono::{DateTime, UTC, ParseError, TimeZone};
use std::io::{Read, Error as IoError};
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;

#[derive(Clone, Debug)]
pub struct PluginSearchQuery<'a> {
    categories: Option<Vec<PluginCategory>>,
    sort: Option<SortType>,
    query: &'a str,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<'a> PluginSearchQuery<'a> {
    pub fn categories(&mut self, categories: &Vec<PluginCategory>) -> &mut Self {
        self.categories = Some(categories.clone());
        self
    }
    pub fn sort_type(&mut self, sort_type: SortType) -> &mut Self {
        self.sort = Some(sort_type);
        self
    }
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn exec(&self) -> Result<Vec<Plugin>, Error> {
        let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
        let mut url = Url::parse("https://ore.spongepowered.org/api/projects").unwrap();
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(ref categories) = self.categories {
                pairs.append_pair("categories", categories.into_iter().map(|x| *x as u8).fold(String::new(), |x, y| x + "," + y.to_string().as_str()).as_str());
            }
            if let Some(ref sort) = self.sort {
                pairs.append_pair("sort", (*sort as u8).to_string().as_str());
            }
            if let Some(ref limit) = self.limit {
                pairs.append_pair("limit", limit.to_string().as_str());
            }
            if let Some(ref offset) = self.offset {
                pairs.append_pair("offset", offset.to_string().as_str());
            }
            pairs.append_pair("q", self.query);
        }
        let mut res = String::new();
        client.get(url).send()?.read_to_string(&mut res)?;
        Ok(serde_json::from_str::<Vec<Plugin>>(&res)?)
    }

}

pub fn search_plugins(query: &str) -> PluginSearchQuery {
    PluginSearchQuery {
        categories: None,
        sort: None,
        query: query,
        limit: None,
        offset: None,
    }
}

//pub fn get_all_plugins() -> Result<Vec<Plugin>, Error> {
//    let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
//    let url = Url::parse("https://ore.spongepowered.org/api/projects").unwrap();
//    let mut res = String::new();
//    client.get(url).send()?.read_to_string(&mut res)?;
//    Ok(serde_json::from_str::<Vec<Plugin>>(&res)?)
//}

pub fn get_plugin(id: &str) -> Result<Plugin, Error> {
    let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
    let url = Url::parse("https://ore.spongepowered.org/api/projects/").unwrap().join(id).map_err(|_| Error::IncorrectIdError)?;
    let mut res = String::new();
    println!("{}", url.to_string());
    client.get(url).send()?.read_to_string(&mut res)?;
    println!("{}", res);
    Ok(serde_json::from_str::<Plugin>(&res)?)
}

pub fn search_versions(id: &str) -> VersionSearchQuery {
    VersionSearchQuery {
        channels: None,
        limit: 10,
        offset: None,
        id: id,
    }
}

#[derive(Clone, Debug)]
pub struct VersionSearchQuery<'a> {
    channels: Option<Vec<&'a str>>,
    limit: u32,
    offset: Option<u32>,
    id: &'a str,
}

impl<'a> VersionSearchQuery<'a> {
    pub fn channels(&mut self, channels: Vec<&'a str>) -> &mut Self {
        self.channels = Some(channels);
        self
    }
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = limit;
        self
    }
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn exec(&self) -> Result<Vec<Version>, Error> {
        let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
        let mut url = Url::parse("https://ore.spongepowered.org/api/projects/").unwrap()
            .join((self.id.to_owned() + "/").as_str()).map_err(|_| Error::IncorrectIdError)?.join("versions").unwrap();
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(ref channels) = self.channels {
                pairs.append_pair("channels", channels.into_iter().fold(String::new(), |x, y| x + "," + y).as_str());
            }
            pairs.append_pair("limit", self.limit.to_string().as_str());
            if let Some(offset) = self.offset {
                pairs.append_pair("offset", offset.to_string().as_str());
            }
        }
        let mut res = String::new();
        client.get(url).send()?.read_to_string(&mut res)?;
        Ok(serde_json::from_str::<Vec<Version>>(&res)?)
    }
}

//pub fn get_all_versions(id: &str) -> Result<Vec<Version>, Error> {
//    let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
//    let url = Url::parse("https://ore.spongepowered.org/api/projects/").unwrap()
//        .join((id.to_owned() + "/").as_str()).map_err(|_| Error::IncorrectIdError)?.join("versions").unwrap();
//    let mut res = String::new();
//    client.get(url).send()?.read_to_string(&mut res)?;
//    Ok(serde_json::from_str::<Vec<Version>>(&res)?)
//}

pub fn get_version(id: &str, version: &str) -> Result<Version, Error> {
    let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
    let url = Url::parse("https://ore.spongepowered.org/api/projects/").unwrap()
        .join((id.to_owned() + "/").as_str()).map_err(|_| Error::IncorrectIdError)?.join("versions/").unwrap()
        .join((version.to_owned() + "/").as_str()).map_err(|_| Error::IncorrectVersionError)?;
    let mut res = String::new();
    client.get(url).send()?.read_to_string(&mut res)?;
    Ok(serde_json::from_str::<Version>(&res)?)
}

pub fn get_users() -> UserQuery {
    UserQuery {
        limit: None,
        offset: None,
    }
}

#[derive(Debug, Clone)]
pub struct UserQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

impl UserQuery {
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn exec(&self) -> Result<Vec<User>, Error> {
        let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
        let mut url = Url::parse("https://ore.spongepowered.org/api/users").unwrap();
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(ref limit) = self.limit {
                pairs.append_pair("limit", limit.to_string().as_str());
            }
            if let Some(ref offset) = self.offset {
                pairs.append_pair("offset", offset.to_string().as_str());
            }
        }
        let mut res = String::new();
        client.get(url).send()?.read_to_string(&mut res)?;
        Ok(serde_json::from_str::<Vec<User>>(&res)?)
    }
}

pub fn get_user(name: &str) -> Result<User, Error> {
    unimplemented!()
}

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    NetError(HyperError),
    ParseError(SerdeError),
    DateError(ParseError),
    IncorrectIdError,
    IncorrectVersionError,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::ParseError(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::DateError(ref err) => err.description(),
            Error::IncorrectIdError => "Incorrect plugin ID format",
            Error::IncorrectVersionError => "Incorrect version format",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::ParseError(ref err) => Some(err),
            Error::NetError(ref err) => Some(err),
            Error::DateError(ref err) => Some(err),
            Error::IncorrectIdError => None,
            Error::IncorrectVersionError => None,
        }
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::ParseError(err)
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Self {
        Error::NetError(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::DateError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::IoError(ref err) => err.fmt(f),
            Error::ParseError(ref err) => err.fmt(f),
            Error::NetError(ref err) => err.fmt(f),
            Error::DateError(ref err) => err.fmt(f),
            Error::IncorrectIdError => write!(f, "Incorrect plugin ID format."),
            Error::IncorrectVersionError => write!(f, "Incorrect version format."),
        }
    }
}

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
        where E: StdError {
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
        category.ok_or(serde::de::Error::custom(r#"Invalid value "category""#))
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
        where E: StdError {
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
    UTC.datetime_from_str(&str, DATETIME_FMT).map_err(DeError::custom)
}
