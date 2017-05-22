/*
 Crate:         ore
 File:          /query/mod.rs
 Module:        ::query
 Visibility:    public
 */

// TODO: documentation

use types::{Plugin, PluginCategory, SortType};
use hyper::Error as HttpError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, self::Error>;

// TODO: documentation
pub trait Query<'a> {
    // TODO: documentation
    /// The return type.
    type Ret;

    // TODO: documentation
    /// Initiate a query.
    fn query(&self, url: &'a str) -> Result<Self::Ret>;
}

// TODO: documentation
#[derive(Debug)]
pub enum Error {
    Http(HttpError),
    InvalidId(String),
    Io(IoError),
    Json(SerdeError),
}
