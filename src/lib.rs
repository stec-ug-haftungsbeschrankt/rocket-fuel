#[macro_use] extern crate serde_derive;
extern crate diesel;
#[macro_use] extern crate log;

pub mod config;
pub mod contexts;
pub mod email;
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

use rocket::config::{Config, ConfigError, Environment};
use rocket_contrib::serve::StaticFiles;

pub fn build_static_files() -> StaticFiles {
    let static_path = format!("../{}/static", get_app_base_path());
    StaticFiles::from(static_path)
}


pub fn build_rocket_config(port: u16) -> Result<Config, ConfigError> {
    let rocket_config = if cfg!(debug_assertions) {
        Config::build(Environment::Development)
            .port(port)
            .finalize()
    } else {
        let template_dir = format!("{}/templates", get_app_base_path());
        let assets_dir = format!("{}/assets", get_app_base_path());

        Config::build(Environment::Production)
            .extra("template_dir", template_dir)
            .extra("assets_dir", assets_dir)
            .address("0.0.0.0")
            .port(port)
            .finalize()
    };
    rocket_config
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

use translations::Translations;


pub fn initialize_translations(section: &str) -> Translations {
    let path = format!("{}/i18n/{}/", get_app_base_path(), section);
    
    let translations = Translations::new(&path);
    translations.unwrap()
}



/*
 * Tera filters
 */

use std::collections::HashMap;
use rocket_contrib::templates::tera::{self, Value};



pub fn prettify_currency(value: Value, _: HashMap<String, Value>) -> tera::Result<Value> {
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
