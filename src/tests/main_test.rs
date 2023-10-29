#[cfg(test)]
mod tests {
    use url::Url;

    #[tokio::test]
    async fn test_insecure() {
        let client = crate::HttpServer::new(
            Url::parse("http://localhost:8080").unwrap(),
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
}
