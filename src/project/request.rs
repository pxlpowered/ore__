/*
 Crate:         ore
 File:          /project/request.rs
 Module:        ::project::request
 Visibility:    public
 */

// TODO: documentation


use project::category::Category;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use request::{Error as RequestError, Request};
use super::Project;
use serde::de::{Error as DeserializeError, Visitor};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

const PROJECTS: &'static str = "/projects";

// TODO: documentation
#[derive(Clone, Debug)]
pub struct ProjectsRequest {
    categories: Option<Vec<Category>>,
    limit: Option<u32>,
    offset: Option<u32>,
    q: Option<String>,
    sort: Option<SortType>,
}

impl ProjectsRequest {
    // TODO: documentation
    pub fn categories(&self) -> Option<Vec<Category>> {
        self.categories.to_owned()
    }

    // TODO: documentation
    pub fn add_category(&mut self, category: &Category) -> &mut Self {

        match self.categories {
            Some(ref mut cats) => cats.push(*category),
            None => self.categories = Some(vec![*category]),
        }

        self
    }

    // TODO: documentation
    pub fn set_categories(&mut self, categories: &Vec<Category>) -> &mut Self {
        self.categories = Some(categories.to_vec());
        self
    }

    // TODO: documentation
    pub fn reset_categories(&mut self) -> &mut Self {
        self.categories = None;
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
        self.offset = Some(offset);
        self
    }

    // TODO: documentation
    pub fn reset_offset(&mut self) -> &mut Self {
        self.offset = None;
        self
    }

    // TODO: documentation
    pub fn query(&self) -> Option<String> {
        self.q.to_owned()
    }

    // TODO: documentation
    pub fn set_query(&mut self, query: String) -> &mut Self {
        self.q = Some(query);
        self
    }

    // TODO: documentation
    pub fn reset_query(&mut self) -> &mut Self {
        self.q = None;
        self
    }

    // TODO: documentation
    pub fn sort_type(&self) -> Option<SortType> {
        self.sort
    }

    // TODO: documentation
    pub fn set_sort_type(&mut self, sort_type: &SortType) -> &mut Self {
        self.sort = Some(*sort_type);
        self
    }

    pub fn reset_sort_type(&mut self) -> &mut Self {
        self.sort = None;
        self
    }
}

impl Default for ProjectsRequest {
    fn default() -> Self {
        ProjectsRequest {
            categories: None,
            limit: None,
            offset: None,
            q: None,
            sort: None,
        }
    }
}

impl<'a> Request<'a> for ProjectsRequest {
    type Ret = Vec<Project>;

    fn request(&self, url: &'a str) -> StdResult<Self::Ret, RequestError> {
        use hyper::{Client, Url};
        use hyper::net::HttpsConnector;
        use hyper_rustls::TlsClient;
        use std::io::Read;

        let mut req_url = Url::parse((url.to_string() + PROJECTS).as_str())?;

        {
            let mut query_pairs = req_url.query_pairs_mut();

            if let Some(ref categories) = self.categories {
                query_pairs.append_pair("categories",
                                        categories
                                            .into_iter()
                                            .map(|c| *c as u8)
                                            .fold(String::new(), |acc, next| {
                    acc + "," + next.to_string().as_str()
                })
                                            .as_str());
            }

            if let Some(ref limit) = self.limit {
                query_pairs.append_pair("limit", (*limit).to_string().as_str());
            }

            if let Some(ref offset) = self.offset {
                query_pairs.append_pair("offset", offset.to_string().as_str());
            }

            if let Some(ref query) = self.q {
                query_pairs.append_pair("q", query);
            }

            if let Some(ref sort) = self.sort {
                query_pairs.append_pair("sort", (*sort as u8).to_string().as_str());
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

// TODO: documentation
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum SortType {
    MostStars,
    MostDownloads,
    MostViews,
    Newest,
    RecentlyUpdated,
}

#[doc(hidden)]
impl<'a> Deserialize<'a> for SortType {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_u8(SortTypeVisitor)
    }
}

impl Display for SortType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
               "{}",
               match *self {
                   SortType::MostStars => "Most Stars",
                   SortType::MostDownloads => "Most Downloads",
                   SortType::MostViews => "Most Views",
                   SortType::Newest => "Newest",
                   SortType::RecentlyUpdated => "Recently Updated",
               })
    }
}

#[doc(hidden)]
impl Serialize for SortType {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

// TODO: documentation
struct SortTypeVisitor;

impl<'a> Visitor<'a> for SortTypeVisitor {
    type Value = SortType;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "an integer between 0-{}", 4)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where E: DeserializeError
    {
        match v {
            0 => Ok(SortType::MostStars),
            1 => Ok(SortType::MostDownloads),
            2 => Ok(SortType::MostViews),
            3 => Ok(SortType::Newest),
            4 => Ok(SortType::RecentlyUpdated),
            _ => Err(DeserializeError::custom(r#"invalid sort type"#)),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where E: DeserializeError
    {
        Self::visit_u8(self, v as u8)
    }
}

pub fn get_plugin(id: &str, url: &str) -> Result<Project, RequestError> {
    use hyper::{Client, Url};
    use hyper::net::HttpsConnector;
    use hyper_rustls::TlsClient;
    use std::io::Read;

    let mut res = String::new();
    Client::with_connector(HttpsConnector::new(TlsClient::new()))
        .get(Url::parse((url.to_string() + PROJECTS).as_str())?.join(&id)
            .map_err(|_| RequestError::InvalidId(id.to_string()))?)
        .send()?
        .read_to_string(&mut res)?;

    Ok(serde_json::from_str::<Project>(res.as_str())?)
}
