#[cfg(test)]
mod tests;
use anyhow::Ok;
use reqwest::{header::HeaderMap, multipart, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct OkJson {
    pub ok: bool,
}

#[derive(Clone)]
pub struct TlsConfig<'a> {
    pub insecure: Option<bool>,
    pub private_chain_bytes: Option<&'a [u8]>,
}

pub struct HttpClient {
    base_url: Url,
    client: Client,
}

impl HttpClient {
    pub fn new(
        base_url: Url,
        tls_config: Option<TlsConfig<'static>>,
        default_headers: Option<HeaderMap>,
    ) -> Self {
        let mut builder = Client::builder();
        if let Some(headers) = default_headers {
            builder = builder.default_headers(headers);
        }
        if base_url.scheme() == "https" {
            builder = builder.use_rustls_tls();

            if let Some(tls_config) = tls_config {
                if let Some(true) = tls_config.insecure {
                    builder = builder.danger_accept_invalid_certs(true);
                } else if let Some(private_chain_bytes) = tls_config.private_chain_bytes {
                    let reqwest_certificate =
                        reqwest::Certificate::from_pem(private_chain_bytes).unwrap();

                    // print!("cert_chain: {:?}", cert_chain.as_slice());

                    builder = builder.add_root_certificate(reqwest_certificate);
                }
            }
        }

        let client = builder.build().unwrap();
        Self { base_url, client }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let url = self.base_url.join(endpoint)?;

        let mut request_builder = self.client.get(url);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let resp = request_builder.send().await?;

        let response = resp.json::<T>().await?;
        Ok(response)
    }

    pub async fn post<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let url = self.base_url.join(endpoint)?;

        let mut request_builder = self.client.post(url).json(body);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let resp = request_builder.send().await?;

        let response = resp.json::<T>().await?;
        Ok(response)
    }

    pub async fn post_file_buffer<T: DeserializeOwned, U: Serialize>(
        &self,
        url: Url,
        name: String,
        bytes: &'static [u8],
        mut multipart_file_name: Option<String>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        if multipart_file_name.is_none() {
            multipart_file_name = Some(String::from("file"));
        }

        let multipart_file_name = multipart_file_name.unwrap();

        println!("UPLOAD {} {}", url.as_str(), &name);

        let part = multipart::Part::bytes(bytes)
            .file_name(name.clone())
            .mime_str("application/octet-stream")?;

        let form = multipart::Form::new().part(multipart_file_name, part);

        self.send_multipart_form(url, form, extra_headers).await
    }

    pub async fn send_multipart_form<T: DeserializeOwned>(
        &self,
        url: Url,
        multipart_form: multipart::Form,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let mut request_builder = self.client.post(url);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let response = request_builder
            .multipart(multipart_form)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }

    pub async fn get_file(
        &self,
        url: &str,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<tokio_util::bytes::Bytes> {
        // println!("GET {url}");
        let mut request_builder = self.client.get(url);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }
        let resp = request_builder.send().await?;
        if resp.status().is_success() {
            let bytes = resp.bytes().await?;
            Ok(bytes)
        } else {
            Err(anyhow::anyhow!("Error downloading file"))
        }
    }
}
