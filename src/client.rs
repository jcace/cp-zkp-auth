use tonic::transport::Channel;

use crate::client::zkp_auth::{auth_client::AuthClient, AuthenticationAnswerRequest};

use self::zkp_auth::{
    AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse,
    RegisterRequest, RegisterResponse,
};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

pub async fn run_client(addr: &str) {
    let mut c = Client::new(addr, "a".to_string()).await;

    let res = c.register("a", 1, 2).await;

    println!("res: {:?}", res);

    let res = c.create_authentication_challenge(4, 5).await;
    println!("res: {:?}", res);

    let res = c.verify_authentication(4, "123".to_string()).await;
    println!("res: {:?}", res);
}

pub struct Client {
    c: AuthClient<Channel>,
    user: String,
}

impl Client {
    pub async fn new(addr: &str, user: String) -> Self {
        let c = AuthClient::connect(format!("http://{}", addr))
            .await
            .unwrap();

        Client { c, user }
    }

    pub async fn register(&mut self, user: &str, y1: i64, y2: i64) -> RegisterResponse {
        let request = tonic::Request::new(RegisterRequest {
            user: user.to_string(),
            y1,
            y2,
        });

        let response = self.c.register(request).await.unwrap();

        response.into_inner()
    }

    pub async fn create_authentication_challenge(
        &mut self,
        r1: i64,
        r2: i64,
    ) -> AuthenticationChallengeResponse {
        let request = tonic::Request::new(AuthenticationChallengeRequest {
            user: self.user.to_string(),
            r1,
            r2,
        });

        let response = self
            .c
            .create_authentication_challenge(request)
            .await
            .unwrap();

        response.into_inner()
    }

    pub async fn verify_authentication(
        &mut self,
        s: i64,
        auth_id: String,
    ) -> AuthenticationAnswerResponse {
        let request = tonic::Request::new(AuthenticationAnswerRequest { auth_id, s });

        let response = self.c.verify_authentication(request).await.unwrap();

        response.into_inner()
    }
}
