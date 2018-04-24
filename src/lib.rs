#![feature(plugin, use_extern_macros, custom_attribute)]
#![recursion_limit = "128"] // https://github.com/diesel-rs/diesel/issues/1127
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use(log)]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

extern crate amqp;
extern crate base64;
extern crate chrono;
extern crate clap;
extern crate epub;
extern crate eui48;
extern crate flate2;
extern crate frank_jwt;
extern crate geo;
extern crate handlebars;
extern crate hex;
extern crate hyper;
extern crate ini;
extern crate language_tags;
extern crate lettre;
extern crate lettre_email;
extern crate log4rs;
extern crate maxminddb;
extern crate md5;
extern crate mime;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate regex;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate serde;
extern crate serde_xml_rs;
extern crate sha2;
extern crate sodiumoxide;
extern crate stardict;
extern crate sys_info;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate validator;

#[macro_use]
pub mod macros;

pub mod app;
pub mod cache;
pub mod console;
pub mod env;
pub mod i18n;
pub mod jwt;
pub mod orm;
pub mod queue;
pub mod result;
pub mod router;
pub mod schema;
pub mod security;
pub mod settings;

pub mod donate;
pub mod erp;
pub mod forum;
pub mod mall;
pub mod nut;
pub mod ops;
pub mod pos;
pub mod reading;
pub mod spree;
pub mod survey;