#[cfg(test)]
mod tests;

use reqwest::{header::HeaderMap, Client};
use url::Url;

#[derive(Clone)]
pub struct TlsConfig<'a> {
    pub insecure: Option<bool>,
    pub private_chain_bytes: Option<&'a [u8]>,
}

pub struct HttpServer {
    base_url: Url,
    client: Client,
}

impl HttpServer {
    pub fn new(
        base_url: Url,
        tls_config: Option<TlsConfig<'static>>,
        default_headers: Option<HeaderMap>,
    ) -> Self {
        let mut builder = Client::builder();
        if base_url.scheme() == "https" {
            builder = builder.use_rustls_tls();
            if tls_config.is_some() {
                let tls_config_content = tls_config.clone().unwrap();

                if tls_config_content.insecure.is_some() {
                    builder = builder.danger_accept_invalid_certs(true);
                    // } else if tls_config_content.private_chain_bytes.is_some() {
                    //     let cert =
                    //         tls::Certificate::from_pem(tls_config_content.private_chain_bytes.unwrap())
                    //             .unwrap();

                    //     builder = builder.add_root_certificate(cert);
                }
            }
        }
        if let Some(headers) = default_headers {
            builder = builder.default_headers(headers);
        }

        let client = builder.build().unwrap();
        Self { base_url, client }
    }
}
