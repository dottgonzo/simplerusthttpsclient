use serde::{self, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct OkJson {
    pub ok: bool,
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::tests::main_test::OkJson;

    #[tokio::test]
    async fn test_http() {
        let client =
            crate::HttpServer::new(Url::parse("http://localhost:3000").unwrap(), None, None);

        let response: anyhow::Result<OkJson> = client.get("/", None).await;

        assert!(response.is_ok());
    }
    #[tokio::test]
    async fn test_https_insecure() {
        let client = crate::HttpServer::new(
            Url::parse("https://localhost:3000").unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(true),
                private_chain_bytes: None,
            }),
            None,
        );

        let response: anyhow::Result<OkJson> = client.get("/", None).await;

        assert!(response.is_ok());
    }
    #[tokio::test]
    async fn test_https_private_tls() {
        let my_cert_bytes = include_bytes!("nodeserver/ca_cert.pem");

        let client = crate::HttpServer::new(
            Url::parse("https://localhost:3000").unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(true),
                private_chain_bytes: Some(my_cert_bytes),
            }),
            None,
        );

        let response: anyhow::Result<OkJson> = client.get("/", None).await;

        assert!(response.is_ok());
    }
}
