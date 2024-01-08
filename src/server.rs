use std::{collections::HashMap, sync::Arc};

use num::{bigint::Sign, BigInt, Zero};
use num_bigint::{RandBigInt, ToBigInt};
use rand_core::{OsRng, RngCore};
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

use crate::{
    cp_params::{self, ChaumPedersenParams},
    db::{AuthChallenge, InMemoryDB, User},
};

use self::zkp_auth::{
    auth_server, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest,
    RegisterResponse,
};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

#[derive(Debug)]
pub struct ZkpAuthService {
    params: ChaumPedersenParams,
    db: Arc<Mutex<InMemoryDB>>,
}

impl ZkpAuthService {
    pub fn new(params: ChaumPedersenParams) -> Self {
        ZkpAuthService {
            params,
            db: Arc::new(Mutex::new(InMemoryDB::new())),
        }
    }
}

#[tonic::async_trait]
impl auth_server::Auth for ZkpAuthService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        log::trace!("request: {:?}", request);

        let r = request.into_inner();

        let y1 = BigInt::from_bytes_be(Sign::Plus, &r.y1);
        let y2 = BigInt::from_bytes_be(Sign::Plus, &r.y2);
        let user_id = r.user;

        let mut db = self.db.lock().await;
        if db.get_user(&user_id).await.is_some() {
            return Err(Status::already_exists(format!(
                "user {} already exists",
                user_id
            )));
        };

        db.create_user(user_id, y1, y2);

        let resp = RegisterResponse {};
        Ok(Response::new(resp))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        log::trace!("request: {:?}", request);

        let r = request.into_inner();
        let user_id = r.user;

        if self.db.lock().await.get_user(&user_id).await.is_none() {
            return Err(Status::not_found(format!(
                "user {} does not exist. please register first",
                user_id
            )));
        };

        let r1 = BigInt::from_bytes_be(Sign::Plus, &r.r1);
        let r2 = BigInt::from_bytes_be(Sign::Plus, &r.r2);

        let c = OsRng.next_u64().to_bigint().unwrap();

        let new_challenge = AuthChallenge::new(user_id, r1, r2, c.to_owned());
        let new_challenge_id = new_challenge.auth_id.clone();

        self.db.lock().await.create_challenge(new_challenge);

        let resp = AuthenticationChallengeResponse {
            auth_id: new_challenge_id,
            c: c.to_bytes_be().1,
        };
        Ok(Response::new(resp))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        log::debug!("Got a request: {:?}", request);
        let r = request.into_inner();

        let s = BigInt::from_bytes_be(Sign::Plus, &r.s);
        let auth_id = r.auth_id;

        if self.db.lock().await.get_challenge(&auth_id).await.is_none() {
            return Err(Status::not_found(format!(
                "challenge {} does not exist. please create a challenge first",
                auth_id
            )));
        };

        let resp = AuthenticationAnswerResponse {
            session_id: "123".to_string(),
        };
        Ok(Response::new(resp))
    }
}

pub async fn run_server(addr: &str) {
    log::info!("starting server on {}", addr);
    let addr = addr.parse().unwrap();

    let params = cp_params::ChaumPedersenParams::new_from_env();
    let service = ZkpAuthService::new(params);

    Server::builder()
        .add_service(auth_server::AuthServer::new(service))
        .serve(addr)
        .await
        .unwrap();
}
