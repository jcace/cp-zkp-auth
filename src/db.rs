use std::{collections::HashMap, sync::Arc};

use num_bigint::BigInt;
use tokio::sync::Mutex;

fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Debug)]
pub struct User {
    pub user_id: String,
    pub y1: BigInt,
    pub y2: BigInt,
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

#[derive(Debug)]
pub struct AuthChallenge {
    pub auth_id: String,
    pub user_id: String,
    pub r1: BigInt,
    pub r2: BigInt,
    pub c: BigInt,
    pub s: Option<BigInt>,
    pub session_id: Option<String>,
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

    pub fn finalize_challenge(&mut self, s: BigInt) {
        let session_id = generate_uuid();
        self.s = Some(s);
        self.session_id = Some(session_id);
    }
}

/// A simple in-memory database for storing users and challenges
#[derive(Debug)]
pub struct InMemoryDB {
    // Note: Mutex is used instead of RWLock for managing access to User and AuthChallenge.
    // This decision is based on the expected workflow of the Chaum-Pedersen protocol, where
    // a single User or AuthChallenge is unlikely to require concurrent read access.
    // Concurrent writes are infrequent, and the simplicity of Mutex is preferred over the
    // slight performance advantage of RWLock in read-heavy scenarios.
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

    /// Create a new user
    pub fn create_user(&mut self, username: String, y1: BigInt, y2: BigInt) {
        self.users.insert(
            username.clone(),
            Arc::new(Mutex::new(User::new(username, y1, y2))),
        );
    }

    /// Create a new challenge
    pub fn create_challenge(&mut self, challenge: AuthChallenge) {
        self.challenges
            .insert(challenge.auth_id.clone(), Arc::new(Mutex::new(challenge)));
    }

    /// Get a user by username
    pub async fn get_user(&self, s: &str) -> Option<&Arc<Mutex<User>>> {
        self.users.get(s)
    }

    /// Get a challenge by its auth_id
    pub async fn get_challenge(&self, s: &str) -> Option<&Arc<Mutex<AuthChallenge>>> {
        self.challenges.get(s)
    }
}
