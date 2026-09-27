use crate::models::Task;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState{
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState{
    pub fn new() -> Self {
        
    let seed = vec![];

        Self {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}
