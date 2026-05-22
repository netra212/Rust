use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::domain::todo::todo::Todo;

pub type AppState = Arc<RwLock<HashMap<i32, Todo>>>;

/**
 * Why Needed?

Your Axum server handles:
    - many requests
    - many threads
    - concurrent access

Without locking:
    - Two requests may modify HashMap simultaneously.

which causes:
    ❌ data race
    ❌ memory corruption

    RwLock Rules:
    - Many requests can READ todos at same time.

    Arc: Atomic Reference Counted Pointer.
    Why Needed?

    Axum shares app state across:
        - routes
        - handlers
        - threads

    Rust ownership rules say:
        - Only one owner normally allowed

    But your app needs MANY owners.
        So Arc allows:
        ✅ shared ownership
        ✅ thread-safe sharing

    With Arc
        Many handlers can safely share same state:
        Handler A
        Handler B
        Handler C
            ↓
        SAME AppState

    Visual Representation
        Arc
        └── RwLock
            └── HashMap
                    ├── 1 -> Todo
                    ├── 2 -> Todo
                    └── 3 -> Todo

    Important Concepts here:
    - Arc: Shared ownership across threads.
    - RwLock: Safe concurrent reading/writing.
    - HashMap: Store Todos in memory.

    In Simple Backend Terms

    This line basically means:

        Create a thread-safe shared in-memory database
        for storing Todos across the whole application.

    Real Backend Analogy
    Imagine:

        HashMap = cupboard storing todos

        RwLock = room security system

        Arc = duplicate keys to room

        many people can LOOK inside room
        only one can CHANGE things
        many handlers have room access
    
    NOTE: Difference Between Mutex and RwLock
    Mutex(Arc<Mutex<T>>): Only ONE access at a time
    RwLock(Arc<RwLock<T>>): Many readers allowed, One writer allowed. Better for APIs because: Reads happen more than writes
 */