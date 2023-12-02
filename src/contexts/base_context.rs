use crate::config::GeneralConfig;
use clap::crate_version;


#[derive(Serialize)]
pub struct BaseContext {
    page_title: String,
    application_title: String,
    language: String,
    version: &'static str
}

impl BaseContext {
    pub fn new(page_title: &'static str, general_config: &GeneralConfig) -> BaseContext {
        BaseContext {
            page_title: page_title.to_string(),
            application_title: general_config.shop_title.clone(),
            language: general_config.language.clone(),
            version: crate_version!()
        }
    }

    pub fn new_raw(page_title: String, application_title: String, language: String) -> BaseContext {
        BaseContext {
            page_title,
            application_title,
            language,
            version: crate_version!()
        }
    }
}