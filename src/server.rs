use std::{collections::HashMap, sync::Arc};

use num::{bigint::Sign, BigInt, Zero};
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

use crate::{
    auth_db::{AuthSession, AuthSessionDb},
    cp_params::{self, ChaumPedersenParams},
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
    db: Arc<Mutex<AuthSessionDb>>,
}

impl ZkpAuthService {
    pub fn new(params: ChaumPedersenParams) -> Self {
        ZkpAuthService {
            params,
            db: Arc::new(Mutex::new(AuthSessionDb::new())),
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

        let mut users = self.db.lock().await;

        if users.contains(&user_id) {
            return Err(Status::already_exists(format!(
                "user {} already exists",
                user_id
            )));
        };

        let new_session = AuthSession::new(user_id.clone(), y1, y2);
        users.insert(user_id, Arc::new(Mutex::new(new_session)));

        let resp = RegisterResponse {};
        Ok(Response::new(resp))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        log::trace!("request: {:?}", request);

        let r = request.into_inner();
        let resp = AuthenticationChallengeResponse {
            auth_id: "123".to_string(),
            c: vec![1, 2, 3],
        };
        Ok(Response::new(resp))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        log::debug!("Got a request: {:?}", request);
        let r = request.into_inner();
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
