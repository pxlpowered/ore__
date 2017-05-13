/*
 Crate:         ore
 File:          /query/mod.rs
 Module:        ::query
 Visibility:    public
 */

//! TODO

use types::{Plugin, PluginCategory, SortType};
use hyper::Error as HttpError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, self::Error>;

/// TODO
pub trait Query<'a> {
    /// The return type.
    /// TODO
    type Ret;

    /// Initiate a query.
    /// TODO
    fn query(&self, url: &'a str) -> Result<Self::Ret>;
}

/// TODO
#[derive(Debug)]
pub enum Error {
    Http(HttpError),
    Io(IoError),
    Json(SerdeError),
}

/// TODO
#[derive(Builder, Clone, Debug, Default)]
#[builder(derive(Debug))]
// TODO: Rename to ProjectsQuery
pub struct PluginsQuery<'a> {
    categories: Option<Vec<PluginCategory>>,
    limit: Option<u32>,
    offset: Option<u32>,
    query: Option<&'a str>,
    sort: Option<SortType>,
}

impl StdError for Error {
    fn cause(&self) -> Option<&StdError> {
        error_impl::error_cause(self)
    }

    fn description(&self) -> &str {
        error_impl::error_desciption(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        error_impl::fmt_display(self, f)
    }
}

impl<'a> PluginsQuery<'a> {
    /// TODO
    pub fn categories(&self) -> Option<Vec<PluginCategory>> {
        self.categories.to_owned()
    }

    /// TODO
    pub fn limit(&self) -> Option<u32> {
        self.limit
    }

    /// TODO
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }

    /// TODO
    pub fn query(&self) -> Option<&'a str> {
        self.query
    }

    /// TODO
    pub fn sort(&self) -> Option<SortType> {
        self.sort
    }
}

impl<'a> Query<'a> for PluginsQuery<'a> {
    type Ret = Vec<Plugin>;
    fn query(&self, url: &'a str) -> Result<Self::Ret> {
        plugins_query_impl::plugins_query(self, url)
    }
}

impl<'a, 'b> From<PluginsQuery<'a>> for PluginsQueryBuilder<'b>
    where 'a: 'b
{
    fn from(plugins_query: PluginsQuery<'a>) -> Self {
        plugins_query_impl::from_builder(plugins_query)
    }
}

mod error_impl;
mod plugins_query_impl;
