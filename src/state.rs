use std::collections::HashMap;
use chrono::{DateTime};

pub struct AppState {
    nudges: HashMap<String, DateTime<Utc>>,
}
