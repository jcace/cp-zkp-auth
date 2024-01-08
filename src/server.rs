use std::collections::HashMap;

use num::{bigint::Sign, BigInt};
use tonic::{transport::Server, Request, Response, Status};

use crate::cp_params::ChaumPedersenParams;

use self::zkp_auth::{
    auth_server, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest,
    RegisterResponse,
};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

#[derive(Debug, Default)]
pub struct ZkpAuthService {}

#[tonic::async_trait]
impl auth_server::Auth for ZkpAuthService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        log::debug!("Got a request: {:?}", request);
        let r = request.into_inner();

        let y1 = BigInt::from_bytes_be(Sign::Plus, &r.y1);
        log::debug!("y1: {:?}", y1);
        // r.y1
        let resp = RegisterResponse {};
        Ok(Response::new(resp))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        log::debug!("Got a request: {:?}", request);
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
    let service = ZkpAuthService::default();

    Server::builder()
        .add_service(auth_server::AuthServer::new(service))
        .serve(addr)
        .await
        .unwrap();
}

struct AuthFlow {
    user: String,
    y1: BigInt,
    y2: BigInt,
    r1: BigInt,
    r2: BigInt,
    c: BigInt,
    s: BigInt,
    session_id: Option<String>,
}

struct AuthServer {
    params: ChaumPedersenParams,
    users: HashMap<String, AuthFlow>,
}

impl AuthServer {
    fn new(params: ChaumPedersenParams) -> Self {
        AuthServer {
            params,
            users: HashMap::new(),
        }
    }
}
