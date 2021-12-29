#![feature(try_trait_v2)]

#[macro_use] extern crate serde_derive;
extern crate diesel;
#[macro_use] extern crate log;
#[macro_use] extern crate rocket;

pub mod auth;
pub mod config;
pub mod contexts;
pub mod email;
pub mod images;
pub mod translations;
pub mod tokens;
pub mod service_error;


pub fn get_app_base_path() -> &'static str {
    let path = if cfg!(debug_assertions) {
        "."
    } else {
        "/usr/share/stec_shop"
    };
    path
}


/*
 * Rocket
 */

use rocket::{fs::FileServer};


pub fn build_static_files() -> FileServer {
    let static_path = format!("{}/static", get_app_base_path());
    FileServer::from(static_path)
}




/*
 * Command Line Interface
 */

use clap::{crate_version, crate_authors, crate_description, Arg, App, ArgMatches};

pub fn cli_handler(title: &str) -> ArgMatches<'static> {
    App::new(title)
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches()
}



/*
 * Translations
 */


use serde_json::Value;
use translations::Translations;


pub fn initialize_translations() -> Translations {
    let path = format!("{}/i18n/", get_app_base_path());
    
    let translations = Translations::new(&path);
    translations.unwrap()
}



/*
 * Tera filters
 */

use std::collections::HashMap;
use rocket_dyn_templates::tera;


pub fn prettify_currency(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let key = match value.as_str() {
        Some(s) => s,
        _ => panic!("Error during translation")
    };

    let currency = match key {
        "EUR" => "€",
        "USD" => "$",
        "GBP" => "£",
        _ => panic!("Invalid Currency")
    };

    let v = serde_json::json!(currency);
    Ok(v)
}
