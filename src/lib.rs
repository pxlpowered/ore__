/*
 Crate:         ore
 File:          /lib.rs
 Module:        ::
 Visibility:    public
 */

extern crate chrono;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod project;
pub mod query;

mod serialize;

mod types;
