/*
 Crate:         ore
 File:          /query/error_impl.rs
 Module:        ::query::error_impl
 Visibility:    private
 */

// TODO: documentation

use hyper::Error as HttpError;
use hyper::error::ParseError as UriParseError;
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Formatter, Display, Result as FmtResult};
use std::io::Error as IoError;
use super::Error;

// TODO: documentation
#[inline]
pub fn error_cause(error: &Error) -> Option<&StdError> {
    match *error {
        Error::Http(ref why) => Some(why),
        Error::Io(ref why) => Some(why),
        Error::Json(ref why) => Some(why),
        _ => None,
    }
}

// TODO: documentation
#[inline]
pub fn error_description(error: &Error) -> &str {
    match *error {
        Error::Http(ref why) => why.description(),
        Error::InvalidId(ref why) => "Invalid project id",
        Error::Io(ref why) => why.description(),
        Error::Json(ref why) => why.description(),
    }
}


// TODO: documentation
#[inline]
pub fn fmt_display(error: &Error, f: &mut Formatter) -> FmtResult {
    match *error {
        Error::Http(ref why) => Display::fmt(why, f),
        Error::Io(ref why) => Display::fmt(why, f),
        Error::InvalidId(ref why) => write!(f, "Invalid plugin id '{id}'", id = why.as_str()),
        Error::Json(ref why) => Display::fmt(why, f),
    }
}

#[doc(hidden)]
impl From<HttpError> for Error {
    fn from(error: HttpError) -> Self {
        Error::Http(error)
    }
}

#[doc(hidden)]
impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::Io(error)
    }
}

#[doc(hidden)]
impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Error::Json(error)
    }
}

#[doc(hidden)]
impl From<UriParseError> for Error {
    fn from(error: UriParseError) -> Self {
        Error::Http(HttpError::from(error))
    }
}
