use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::Todo;

pub type Database = Arc<Mutex<HashMap<u64, Todo>>>;

pub fn init_db() -> Database {
    Arc::new(Mutex::new(HashMap::new()))
}
