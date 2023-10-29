#[cfg(test)]
mod tests {
    use url::Url;

    const TEST_URL: &str = "localhost:3000";

    #[tokio::test]
    async fn test_http() {
        let url_string = String::from("http://") + TEST_URL;

        let client = crate::HttpClient::new(Url::parse(&url_string).unwrap(), None, None);

        let response: anyhow::Result<crate::OkJson> = client.get("/", None).await;

        assert!(response.is_ok());
    }
    #[tokio::test]
    async fn test_https_insecure() {
        let url_string = String::from("https://") + TEST_URL;

        let client = crate::HttpClient::new(
            Url::parse(&url_string).unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(true),
                private_chain_bytes: None,
            }),
            None,
        );

        let response: anyhow::Result<crate::OkJson> = client.get("/", None).await;

        assert!(response.is_ok());
    }
    #[tokio::test]
    async fn test_https_private_tls() {
        let url_string = String::from("https://") + TEST_URL;

        let my_cert_bytes = include_bytes!("nodeserver/ca_cert.pem");

        let client = crate::HttpClient::new(
            Url::parse(&url_string).unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(true),
                private_chain_bytes: Some(my_cert_bytes),
            }),
            None,
        );

        let response: anyhow::Result<crate::OkJson> = client.get("/", None).await;

        assert!(response.is_ok());
    }
}
