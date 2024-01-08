use std::{collections::HashMap, sync::Arc};

use num::Zero;
use num_bigint::BigInt;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AuthSession {
    user: String,
    y1: BigInt,
    y2: BigInt,
    r1: Option<BigInt>,
    r2: Option<BigInt>,
    c: Option<BigInt>,
    s: Option<BigInt>,
    session_id: Option<String>,
}

impl AuthSession {
    pub fn new(user: String, y1: BigInt, y2: BigInt) -> Self {
        AuthSession {
            user,
            y1,
            y2,
            r1: None,
            r2: None,
            c: None,
            s: None,
            session_id: None,
        }
    }
}
#[derive(Debug)]
pub struct AuthSessionDb {
    db: HashMap<String, HashMap<String, AuthSession>>, // ! User can have multiple sessions
}

impl AuthSessionDb {
    pub fn new() -> Self {
        AuthSessionDb { db: HashMap::new() }
    }

    pub fn contains(&self, s: &str) -> bool {
        self.db.contains_key(s)
    }

    pub fn insert(&mut self, s: String, a: Arc<Mutex<AuthSession>>) {
        self.db.insert(s, a);
    }
}
