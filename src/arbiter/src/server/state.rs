

use actix::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
}

impl AppState {
    pub fn new(app_name: &str) -> Self {
        AppState {
            app_name: app_name.to_string(),
        }
    }
}
