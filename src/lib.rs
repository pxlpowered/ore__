/*
 Crate:         ore
 File:          /lib.rs
 Module:        ::
 Visibility:    public
 */

extern crate chrono;
#[macro_use]
extern crate derive_builder;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod query;

mod api;
mod types;

pub use types::{Plugin, PluginCategory, SortType, ShortUser, Channel, Version, Dependency, User};
pub use api::{UserQuery, Error, PluginSearchQuery, VersionSearchQuery};
pub use api::{search_plugins, search_versions, get_plugin, get_users, get_user, get_version};
