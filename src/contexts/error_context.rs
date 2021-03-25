use crate::contexts::base_context::BaseContext;
use crate::config::GeneralConfig;

use rocket::State;


#[derive(Serialize)]
pub struct CatchContext {
    base: BaseContext,
    path: String
}

impl CatchContext {
    pub fn new(path: String, config: State<GeneralConfig>) -> CatchContext {
        CatchContext {
            base: BaseContext::new("404 Error", &config),
            path
        }
    }
}
