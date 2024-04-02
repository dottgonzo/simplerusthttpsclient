#[cfg(test)]
mod tests {

    use url::Url;

    const TEST_URL: &str = "localhost:3000";

    #[tokio::test]
    #[cfg(not(feature = "tls"))]
    async fn test_http() {
        let url_string = String::from("http://") + TEST_URL;

        let client = crate::HttpClient::new(Url::parse(&url_string).unwrap(), None);

        let response: anyhow::Result<crate::OkJson> = client.get("/", None).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "tls")]
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

        let response: anyhow::Result<crate::OkJson> = client.get_json("/", None).await;

        assert!(response.is_ok());
    }
    #[tokio::test]
    #[cfg(feature = "tls")]
    async fn test_https_insecure_fail() {
        let url_string = String::from("https://") + TEST_URL;

        let client = crate::HttpClient::new(
            Url::parse(&url_string).unwrap(),
            Some(crate::TlsConfig {
                insecure: None,
                private_chain_bytes: None,
            }),
            None,
        );

        let response: anyhow::Result<crate::OkJson> = client.get_json("/", None).await;

        assert!(response.is_err());
    }
    #[tokio::test]
    #[cfg(feature = "tls")]
    async fn test_https_private_tls() {
        let url_string = String::from("https://") + TEST_URL;

        let my_cert_bytes = include_bytes!("nodeserver/ca_cert.pem");

        let client = crate::HttpClient::new(
            Url::parse(&url_string).unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(false),
                private_chain_bytes: Some(my_cert_bytes.to_vec()),
            }),
            None,
        );

        let response: anyhow::Result<crate::OkJson> = client.get_json("/", None).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "tls")]

    async fn test_send_buffer() {
        let url_string = String::from("https://") + TEST_URL;

        let url_post_string = String::from("https://") + TEST_URL + "/";

        let my_cert_bytes = include_bytes!("nodeserver/ca_cert.pem");

        let client = crate::HttpClient::new(
            Url::parse(&url_string).unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(true),
                private_chain_bytes: None,
            }),
            None,
        );

        let response = client
            .post_file_buffer(
                url::Url::parse(&url_post_string).unwrap(),
                String::from("test.pem"),
                my_cert_bytes,
                None,
                None,
            )
            .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "tls")]
    #[cfg(feature = "async-fs")]
    async fn get_archive_to_dir() {
        let url_get_string =
            String::from("https://crates.io/api/v1/crates/simplerusthttpsclient/0.0.1/download");

        let storage_path = std::path::Path::new("/tmp");

        let client = crate::HttpClient::new(
            Url::parse(&url_get_string).unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(true),
                private_chain_bytes: None,
            }),
            None,
        );

        let response = client
            .get_archive_to_dir(
                url::Url::parse(&url_get_string).unwrap(),
                &crate::ArchiveType::Gzip,
                storage_path,
                None,
            )
            .await;

        if response.is_err() {
            println!("Error: {:?}", &response.err());
        } else {
            assert!(response.is_ok());
        }
    }

    #[tokio::test]
    #[cfg(feature = "tls")]
    #[cfg(feature = "async-fs")]
    async fn spawn_get_archive_to_dir() {
        let url_get_string =
            String::from("https://crates.io/api/v1/crates/simplerusthttpsclient/0.0.1/download");

        let storage_path = std::path::Path::new("/tmp");

        let client = crate::HttpClient::new(
            Url::parse(&url_get_string).unwrap(),
            Some(crate::TlsConfig {
                insecure: Some(true),
                private_chain_bytes: None,
            }),
            None,
        );

        let cloned_client = client.clone();
        tokio::spawn(async move {
            let a = cloned_client
                .get_archive_to_dir(
                    url::Url::parse(&url_get_string).unwrap(),
                    &crate::ArchiveType::Gzip,
                    storage_path,
                    None,
                )
                .await;

            assert!(a.is_ok());
        });

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
