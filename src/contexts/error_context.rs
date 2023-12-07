use crate::contexts::base_context::BaseContext;
use crate::config::GeneralConfig;


#[derive(Serialize)]
pub struct CatchContext {
    base: BaseContext,
    path: String
}

impl CatchContext {
    pub fn new(path: String, config: &GeneralConfig) -> CatchContext {
        CatchContext {
            base: BaseContext::new("404 Error", config),
            path
        }
    }

    pub fn new_raw(path: String, application_title: String, language: String) -> CatchContext {
        CatchContext {
            base: BaseContext::new_raw("404 Error".to_string(), application_title, language),
            path
        }
    }
}
