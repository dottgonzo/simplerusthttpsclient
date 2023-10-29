#[cfg(test)]
mod tests;

use reqwest::{header::HeaderMap, Client};
use rustls::RootCertStore;
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
                } else if tls_config_content.private_chain_bytes.is_some() {
                    let cert = BufReader::new(File::open(cert_path)?);
                    let cert_chain = rustls::internal::pemfile::certs(&cert).map_err(|_| {
                        Error::new(
                            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                            "Impossibile leggere il certificato self-signed",
                        )
                    })?;

                    // Crea un nuovo config Client di Rustls.
                    let mut config = ClientConfig::new();
                    config.root_store.add(&cert_chain[0]).map_err(|_| {
                        Error::new(
                            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                            "Impossibile aggiungere il certificato self-signed",
                        )
                    })?;

                    builder = builder.use_rustls_tls_with_config(config);
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
