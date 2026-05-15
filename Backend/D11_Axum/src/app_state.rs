/**
* Why shared state ?
* Because every route needs access to todos.
* Example:
   create todo
   delete todo
   read todo
   All need same data source.
*/
use crate::domain::todo::todo::Todo;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
pub type AppState = Arc<RwLock<HashMap<i32, Todo>>>;
