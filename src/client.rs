use num::{bigint::ToBigInt, traits::ToBytes};
use num_bigint::{BigInt, Sign};
use rand_core::{OsRng, RngCore};
use tonic::transport::Channel;

use crate::{
    client::zkp_auth::{auth_client::AuthClient, AuthenticationAnswerRequest},
    cp_params,
};

use self::zkp_auth::{
    AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse,
    RegisterRequest, RegisterResponse,
};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

pub async fn run_client(addr: &str, user: &str, secret: &i64) {
    let mut client = Client::new(addr, user.to_string()).await;

    let x = secret.to_bigint().unwrap();
    let params = cp_params::ChaumPedersenParams::new_from_env();
    let (y1, y2) = params.y1_y2(&x);

    let res = client
        .register(user, y1.to_bytes_be().1, y2.to_bytes_be().1)
        .await;

    log::trace!("RegisterResponse: {:?}", res);

    let k = OsRng.next_u64().to_bigint().unwrap();
    let (r1, r2) = params.r1_r2(&k);

    let res = client
        .create_authentication_challenge(r1.to_be_bytes().to_vec(), r2.to_be_bytes().to_vec())
        .await;

    log::trace!("AuthenticationChallengeResponse: {:?}", res);

    let auth_id = res.auth_id;
    let c = BigInt::from_bytes_be(Sign::Plus, &res.c);
    let s = params.s(&k, &c, &x);

    let res = client
        .verify_authentication(s.to_be_bytes().to_vec(), auth_id)
        .await;

    log::trace!("AuthenticationAnswerResponse: {:?}", res);

    println!("Authentication successful. Session ID: {}", res.session_id);
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

    pub async fn register(&mut self, user: &str, y1: Vec<u8>, y2: Vec<u8>) -> RegisterResponse {
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
        r1: Vec<u8>,
        r2: Vec<u8>,
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
        s: Vec<u8>,
        auth_id: String,
    ) -> AuthenticationAnswerResponse {
        let request = tonic::Request::new(AuthenticationAnswerRequest { auth_id, s });

        let response = self.c.verify_authentication(request).await.unwrap();

        response.into_inner()
    }
}
