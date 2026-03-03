use crate::models::Pipeline;
use std::sync::Mutex;

pub struct AppState {
    pub pipelines: Mutex<Vec<Pipeline>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            pipelines: Mutex::new(Vec::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
