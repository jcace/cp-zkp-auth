use tonic::{transport::Server, Request, Response, Status};

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
            c: 1i64,
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
