/*
 Crate:         ore
 File:          /query/impls.rs
 Module:        ::query::impls
 Visibility:    private
 */

//! TODO

use types::Plugin;
use hyper_rustls::TlsClient;
use hyper::{Client, Url};
use hyper::net::HttpsConnector;
use serde_json;
use std::io::Read;
use super::{Error, PluginsQuery, PluginsQueryBuilder};

const PROJECTS: &'static str = "/projects";

/// TODO
#[inline]
pub fn from_builder<'a, 'b>(plugins_query: PluginsQuery<'a>) -> PluginsQueryBuilder<'b>
    where 'a: 'b
{
    let mut tmp = PluginsQueryBuilder::default();
    &mut tmp.categories(plugins_query.categories());
    &mut tmp.limit(plugins_query.limit());
    &mut tmp.offset(plugins_query.offset());
    &mut tmp.query(plugins_query.query());
    &mut tmp.sort(plugins_query.sort());
    tmp
}

/// TODO
#[inline]
pub fn plugins_query(plugin_query: &PluginsQuery, url: &str) -> Result<Vec<Plugin>, Error> {
    let mut req_url: Url;

    {
        let mut base_url = url.to_string();
        base_url.push_str(PROJECTS);

        req_url = Url::parse(&base_url)?;
    }

    {
        let mut query_pairs = req_url.query_pairs_mut();

        if let Some(ref categories) = plugin_query.categories {
            query_pairs.append_pair("categories",
                                    categories
                                        .into_iter()
                                        .map(|c| *c as u8)
                                        .fold(String::new(), |acc, next| {
                acc + "," + next.to_string().as_str()
            })
                                        .as_str());
        }

        if let Some(ref limit) = plugin_query.limit {
            query_pairs.append_pair("limit", (*limit).to_string().as_str());
        }

        if let Some(ref offset) = plugin_query.offset {
            query_pairs.append_pair("offset", offset.to_string().as_str());
        }

        if let Some(ref query) = plugin_query.query {
            query_pairs.append_pair("q", query);
        }

        if let Some(ref sort) = plugin_query.sort {
            query_pairs.append_pair("sort", (*sort as u8).to_string().as_str());
        }
    }

    let mut res = String::new();

    {
        let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
        client.get(req_url).send()?.read_to_string(&mut res)?;
    }

    Ok(serde_json::from_str::<Vec<Plugin>>(res.as_str())?)
}
