use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::domain::todo::todo::Todo;
pub type AppState = Arc<RwLock<HashMap<i32, Todo>>>;
