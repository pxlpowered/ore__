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
