#[cfg(test)]
mod tests {
    use url::Url;

    #[tokio::test]
    async fn test_http() {
        let client =
            crate::HttpServer::new(Url::parse("http://localhost:3000").unwrap(), None, None);

        let response = client
            .client
            .get(client.base_url.join("/").unwrap())
            .send()
            .await
            .unwrap();

        println!("response: {:?}", response);
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

        let response = client
            .client
            .get(client.base_url.join("/").unwrap())
            .send()
            .await
            .unwrap();

        println!("response: {:?}", response);
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

        let response = client
            .client
            .get(client.base_url.join("/").unwrap())
            .send()
            .await
            .unwrap();

        println!("response: {:?}", response);
    }
}
