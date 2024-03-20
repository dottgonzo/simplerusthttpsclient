#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::{fs::File, path::Path};

use reqwest::Response;
use reqwest::{header::HeaderMap, multipart, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Read;
use std::io::Write;
#[cfg(feature = "async-fs")]
use tokio::io::AsyncWriteExt;
use url::Url;

#[derive(Debug, Clone)]
pub enum ArchiveType {
    Zip,
    Gzip,
    Tar,
}
impl ArchiveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArchiveType::Zip => "zip",
            ArchiveType::Gzip => "gzip",
            ArchiveType::Tar => "tar",
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OkJson {
    pub ok: bool,
}
#[cfg(feature = "tls")]
#[derive(Clone, Debug)]
pub struct TlsConfig {
    pub insecure: Option<bool>,
    pub private_chain_bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct HttpClient {
    base_url: Url,
    client: Client,
}

impl HttpClient {
    #[cfg(feature = "tls")]

    pub fn new(
        base_url: Url,
        tls_config: Option<TlsConfig>,
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
                        reqwest::Certificate::from_pem(&private_chain_bytes).unwrap();

                    // print!("cert_chain: {:?}", cert_chain.as_slice());

                    builder = builder.add_root_certificate(reqwest_certificate);
                }
            }
        }

        let client = builder.build().unwrap();
        Self { base_url, client }
    }

    #[cfg(not(feature = "tls"))]

    pub fn new(base_url: Url, default_headers: Option<HeaderMap>) -> Self {
        let mut builder = Client::builder();
        if let Some(headers) = default_headers {
            builder = builder.default_headers(headers);
        }
        if base_url.scheme() == "https" {
            panic!("https is not supported in this build");
        }

        let client = builder.build().unwrap();
        Self { base_url, client }
    }

    pub async fn get(
        &self,
        endpoint: &str,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Response> {
        let url = self.base_url.join(endpoint)?;

        let mut request_builder = self.client.get(url);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let resp = request_builder.send().await?;

        Ok(resp)
    }

    pub async fn get_json<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let resp = self.get(endpoint, extra_headers).await?;

        let response = resp.json::<T>().await?;
        Ok(response)
    }
    pub async fn post<U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Response> {
        let url = self.base_url.join(endpoint)?;

        let mut request_builder = self.client.post(url).json(body);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let resp = request_builder.send().await?;

        Ok(resp)
    }
    pub async fn post_json<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let resp = self.post(endpoint, body, extra_headers).await?;

        let response = resp.json::<T>().await?;
        Ok(response)
    }

    pub async fn patch<U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Response> {
        let url = self.base_url.join(endpoint)?;

        let mut request_builder = self.client.patch(url).json(body);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let resp = request_builder.send().await?;

        Ok(resp)
    }

    pub async fn patch_json<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let resp = self.patch(endpoint, body, extra_headers).await?;

        let response = resp.json::<T>().await?;
        Ok(response)
    }

    pub async fn put<U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Response> {
        let url = self.base_url.join(endpoint)?;

        let mut request_builder = self.client.put(url).json(body);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let resp = request_builder.send().await?;

        Ok(resp)
    }

    pub async fn put_json<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &str,
        body: &U,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let resp = self.put(endpoint, body, extra_headers).await?;

        let response = resp.json::<T>().await?;
        Ok(response)
    }

    pub async fn delete(
        &self,
        endpoint: &str,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Response> {
        let url = self.base_url.join(endpoint)?;

        let mut request_builder = self.client.delete(url);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let resp = request_builder.send().await?;

        Ok(resp)
    }

    pub async fn delete_json<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T> {
        let resp = self.delete(endpoint, extra_headers).await?;

        let response = resp.json::<T>().await?;
        Ok(response)
    }
    #[cfg(feature = "async-fs")]

    pub async fn post_file_as_zip(
        &self,
        url: Url,
        path: &Path,
        multipart_file_name: Option<String>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<()> {
        let tmp_dir = tempfile::tempdir()?;
        let zip_file_name = String::from("test.zip");
        let tmp_file_buff = tmp_dir.path().join(zip_file_name.clone());
        let tmp_file_path = Path::new(&tmp_file_buff).to_owned();
        let cloned_path = tmp_file_path.clone();
        let to_be_zipped = path.to_owned();
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            let mut zip = zip::ZipWriter::new(std::fs::File::create(tmp_file_path)?);
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);
            zip.start_file(zip_file_name, options)?;
            zip.write_all(&std::fs::read(to_be_zipped)?)?;
            zip.finish()?;
            Ok(())
        })
        .await??;
        self.post_file_path(
            url,
            cloned_path.as_path(),
            multipart_file_name,
            extra_headers,
        )
        .await
    }
    #[cfg(feature = "async-fs")]

    pub async fn post_folder_as_zip(
        &self,
        url: Url,
        path: &Path,
        multipart_file_name: Option<String>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<()> {
        let tmp_dir = tempfile::tempdir()?;
        let tmp_file_buff = tmp_dir.path().join("test.zip");
        let tmp_file_path = Path::new(&tmp_file_buff).to_owned();
        let cloned_path = tmp_file_path.clone();
        let to_be_zipped = path.to_owned();
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            let mut zip = zip::ZipWriter::new(std::fs::File::create(tmp_file_path)?);
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);
            for file in walkdir::WalkDir::new(to_be_zipped) {
                let file = file?;
                let path = file.path();
                let name = path.strip_prefix(path)?.to_str().unwrap();
                if path.is_file() {
                    zip.start_file(name, options)?;
                    zip.write_all(&std::fs::read(path)?)?;
                } else if path.is_dir() {
                    zip.add_directory(name, options)?;
                }
            }
            zip.finish()?;
            Ok(())
        })
        .await??;
        self.post_file_path(
            url,
            cloned_path.as_path(),
            multipart_file_name,
            extra_headers,
        )
        .await
    }

    pub async fn post_file_path(
        &self,
        url: Url,
        path: &Path,
        multipart_file_name: Option<String>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<()> {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        let mut file = File::open(path)?;

        // Create a buffer to store the file's contents
        let mut buffer = Vec::new();

        // Read the file's contents into the buffer
        file.read_to_end(&mut buffer)?;
        let multipart = multipart_file_name;
        let headers = extra_headers;
        let url_address = url;

        let static_buffer: &'static [u8] = Box::leak(buffer.into_boxed_slice());

        self.post_file_buffer(url_address, file_name, static_buffer, multipart, headers)
            .await
    }

    pub async fn post_file_buffer(
        &self,
        url: Url,
        name: String,
        bytes: &'static [u8],
        mut multipart_file_name: Option<String>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<()> {
        if multipart_file_name.is_none() {
            multipart_file_name = Some(String::from("file"));
        }

        let multipart_file_name = multipart_file_name.unwrap();

        let part = multipart::Part::bytes(bytes)
            .file_name(name.clone())
            .mime_str("application/octet-stream")?;

        let form = multipart::Form::new().part(multipart_file_name, part);

        self.send_multipart_form(url, form, extra_headers).await
    }

    pub async fn send_multipart_form(
        &self,
        url: Url,
        multipart_form: multipart::Form,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<()> {
        let mut request_builder = self.client.post(url);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }

        let response = request_builder.multipart(multipart_form).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Error uploading file"))
        }

        // .json::<T>()
        // .await?;
    }

    pub async fn get_file_buffer(
        &self,
        url: Url,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Option<tokio_util::bytes::Bytes>> {
        let mut request_builder = self.client.get(url);

        if let Some(headers) = extra_headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value);
            }
        }
        let resp = request_builder.send().await?;
        if resp.status().is_success() {
            let mut answer: Option<tokio_util::bytes::Bytes> = None;
            let bytes_answer = resp.bytes().await;
            if let Ok(bytes_answer) = bytes_answer {
                if !bytes_answer.is_empty() {
                    answer = Some(bytes_answer);
                }
            }
            Ok(answer)
        } else {
            Err(anyhow::anyhow!("Error downloading file"))
        }
    }
    #[cfg(feature = "async-fs")]

    pub async fn get_file_to_path(
        &self,
        url: Url,
        path: &Path,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Option<PathBuf>> {
        let file_buffer = self.get_file_buffer(url, extra_headers).await?;
        if file_buffer.is_some() {
            let mut file = tokio::fs::File::create(&path).await?;
            file.write_all(&file_buffer.unwrap()).await?;

            Ok(Some(path.to_path_buf()))
        } else {
            Ok(None)
        }
    }
    #[cfg(not(feature = "async-fs"))]
    pub async fn get_file_to_path(
        &self,
        url: Url,
        path: &Path,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Option<PathBuf>> {
        let file_buffer = self.get_file_buffer(url, extra_headers).await?;
        if file_buffer.is_some() {
            let mut file = File::create(path)?;
            file.write_all(file_buffer.unwrap().as_ref())?;

            Ok(Some(path.to_path_buf()))
        } else {
            Ok(None)
        }
    }
    #[cfg(feature = "async-fs")]
    pub async fn get_archive_to_dir(
        &self,
        url: Url,
        archive_type: &ArchiveType,
        dir: &Path,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Option<PathBuf>> {
        let file_buffer = self.get_file_buffer(url, extra_headers).await?;
        if let Some(file_buffer) = file_buffer {
            match archive_type {
                ArchiveType::Zip => {
                    let dir = dir.to_owned();
                    tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(file_buffer))?;
                        archive.extract(dir)?;

                        Ok(())
                    })
                    .await??;
                }
                ArchiveType::Tar => {
                    let dir = dir.to_owned();
                    tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                        let mut archive = tar::Archive::new(std::io::Cursor::new(file_buffer));
                        archive.unpack(dir)?;

                        Ok(())
                    })
                    .await??;
                }
                ArchiveType::Gzip => {
                    let dir = dir.to_owned();
                    tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
                            std::io::Cursor::new(file_buffer),
                        ));
                        archive.unpack(dir)?;
                        Ok(())
                    })
                    .await??;
                }
            }

            Ok(Some(dir.to_path_buf()))
        } else {
            Ok(None)
        }
    }
}
