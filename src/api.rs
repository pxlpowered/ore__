use hyper::{Client, Url};
use hyper::error::Error as HyperError;
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt::{Formatter, Result as FmtResult, Display};
use chrono::ParseError;
use std::io::{Read, Error as IoError};
use super::*;

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
