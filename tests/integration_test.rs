#[cfg(test)]
mod integration_tests {
    use num_bigint::ToBigInt;
    use std::time::Duration;
    use zkp_auth::client::run_client_auth_check;
    use zkp_auth::cp_params::ChaumPedersenParams;
    use zkp_auth::server::run_server;

    static SERVER_ADDR: &str = "127.0.0.1:8181";
    static TEST_USER: &str = "test_user";
    static TEST_PASSWORD: &i64 = &64;

    fn create_test_params() -> ChaumPedersenParams {
        // Example parameters (usually these should be large prime numbers)
        let p = 10009.to_bigint().unwrap();
        let q = 5004.to_bigint().unwrap();
        let g = 2.to_bigint().unwrap();
        let h = 3.to_bigint().unwrap();

        ChaumPedersenParams::new(p, q, g, h)
    }

    #[tokio::test]
    async fn test_end_to_end_functionality() {
        let test_params = create_test_params();
        // Spin up the server
        let server_thread = tokio::spawn(run_server(SERVER_ADDR, test_params.clone()));

        // Wait for the server to start
        tokio::time::sleep(Duration::from_secs(1)).await;

        let res =
            run_client_auth_check(SERVER_ADDR, TEST_USER, TEST_PASSWORD, test_params.clone()).await;

        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());

        // Stop the server
        server_thread.abort();
    }
}
