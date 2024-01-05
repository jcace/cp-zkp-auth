use crate::client::zkp_auth::auth_client::AuthClient;

use self::zkp_auth::RegisterRequest;

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

pub async fn run_client(addr: &str) {
    let mut client = AuthClient::connect(format!("http://{}", addr))
        .await
        .unwrap();

    let request = tonic::Request::new(RegisterRequest {
        user: "a".to_string(),
        y1: 1,
        y2: 2,
    });

    let response = client.register(request).await.unwrap();

    println!("RESPONSE={:?}", response);
}
