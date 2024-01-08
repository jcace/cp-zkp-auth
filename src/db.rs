use std::{collections::HashMap, sync::Arc};

use num_bigint::BigInt;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AuthChallenge {
    pub auth_id: String,
    pub user_id: String,
    r1: BigInt,
    r2: BigInt,
    c: BigInt,
    s: Option<BigInt>,
    session_id: Option<String>,
}

#[derive(Debug)]
pub struct User {
    user_id: String,
    y1: BigInt,
    y2: BigInt,
}

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

impl AuthChallenge {
    pub fn new(user_id: String, r1: BigInt, r2: BigInt, c: BigInt) -> Self {
        let auth_id = generate_uuid();
        AuthChallenge {
            auth_id,
            user_id,
            r1,
            r2,
            c,
            s: None,
            session_id: None,
        }
    }

    pub fn finalize_challenge(&mut self, s: BigInt, session_id: String) {
        self.s = Some(s);
        self.session_id = Some(session_id);
    }
}

#[derive(Debug)]
pub struct InMemoryDB {
    users: HashMap<String, Arc<Mutex<User>>>,
    challenges: HashMap<String, Arc<Mutex<AuthChallenge>>>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB {
            users: HashMap::new(),
            challenges: HashMap::new(),
        }
    }

    pub fn create_user(&mut self, username: String, y1: BigInt, y2: BigInt) {
        self.users.insert(
            username.clone(),
            Arc::new(Mutex::new(User::new(username, y1, y2))),
        );
    }

    pub fn create_challenge(&mut self, challenge: AuthChallenge) {
        self.challenges
            .insert(challenge.auth_id.clone(), Arc::new(Mutex::new(challenge)));
    }

    pub async fn get_user(&self, s: &str) -> Option<&Arc<Mutex<User>>> {
        self.users.get(s)
    }

    pub async fn get_challenge(&self, s: &str) -> Option<&Arc<Mutex<AuthChallenge>>> {
        self.challenges.get(s)
    }

    // pub fn delete(&mut self, s: &str) {
    //     self.users.remove(s);
    // }
}

impl User {
    pub fn new(user: String, y1: BigInt, y2: BigInt) -> Self {
        User {
            user_id: user,
            y1,
            y2,
        }
    }
}
