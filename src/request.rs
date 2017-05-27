/*
 Crate:         ore
 File:          /request.rs
 Module:        ::request
 Visibility:    public
 */

// TODO: documentation

use hyper::Error as HttpError;
use hyper::error::ParseError as UriParseError;
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

// TODO: documentation
pub trait Request<'a> {
    // TODO: documentation
    /// The return type.
    type Ret;

    // TODO: documentation
    /// Initiate a query.
    fn request(&self, url: &'a str) -> StdResult<Self::Ret, self::Error>;
}

// TODO: documentation
#[derive(Debug)]
pub enum Error {
    Http(HttpError),
    InvalidId(String),
    Io(IoError),
    Json(JsonError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Http(ref why) => why.fmt(f),
            Error::InvalidId(ref why) => write!(f, "invalid project id {}", why),
            Error::Io(ref why) => why.fmt(f),
            Error::Json(ref why) => why.fmt(f),
        }
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Error::Http(err)
    }
}

impl From<UriParseError> for Error {
    fn from(err: UriParseError) -> Self {
        Error::Http(HttpError::Uri(err))
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref why) => why.description(),
            Error::InvalidId(..) => "Invalid project id found",
            Error::Io(ref why) => why.description(),
            Error::Json(ref why) => why.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Http(ref why) => Some(why),
            Error::Io(ref why) => Some(why),
            Error::Json(ref why) => Some(why),
            _ => None,
        }
    }
}
