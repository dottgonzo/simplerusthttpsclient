#[cfg(test)]
mod tests;

use std::{fs::File, path::Path};

use reqwest::{header::HeaderMap, multipart, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Read;
use std::io::Write;
use tokio::io::AsyncWriteExt;
use url::Url;

pub enum ArchiveType {
    Zip,
    Tar,
    Gzip,
}

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

    pub async fn get_file_buffer(
        &self,
        url: Url,
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

    pub async fn get_file_to_path(
        &self,
        url: Url,
        path: &Path,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<()> {
        // println!("GET {url}");
        let file_buffer = self.get_file_buffer(url, extra_headers).await?;
        let mut file = tokio::fs::File::create(&path).await?;

        file.write_all(&file_buffer).await?;

        Ok(())
    }

    pub async fn get_archive_to_dir(
        &self,
        url: Url,
        archive_type: &ArchiveType,
        dir: &Path,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<()> {
        // println!("GET {url}");

        let file_buffer = self.get_file_buffer(url, extra_headers).await?;

        match archive_type {
            ArchiveType::Zip => {
                let mut archive = zip::ZipArchive::new(std::io::Cursor::new(file_buffer))?;

                // Iterate over each file inside the ZIP
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let outpath = Path::new(dir).join(file.mangled_name());

                    if file.name().ends_with('/') {
                        // Create a directory if it's a folder
                        tokio::fs::create_dir_all(&outpath).await?;
                    } else {
                        // Ensure the parent folder exists
                        if let Some(parent) = outpath.parent() {
                            tokio::fs::create_dir_all(parent).await?;
                        }

                        // Extract the file and write to the output path
                        let mut outfile = tokio::fs::File::create(&outpath).await?;
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer)?;
                        outfile.write_all(&buffer).await?;
                    }
                }
            }
            ArchiveType::Tar => {
                let buffer = file_buffer.clone(); // Clone the buffer so it can be moved into the closure

                let original_dir = dir.to_owned(); // Clone the dir so it can be moved into the closure

                tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                    let mut archive = tar::Archive::new(std::io::Cursor::new(buffer));

                    for entry in archive.entries()? {
                        let mut file = entry?;
                        let outpath = original_dir.join(file.path()?); // Directly use the passed 'dir' parameter

                        if file.header().entry_type().is_dir() {
                            std::fs::create_dir_all(&outpath)?;
                        } else {
                            // Ensure the parent folder exists
                            if let Some(parent) = outpath.parent() {
                                std::fs::create_dir_all(parent)?;
                            }

                            // Extract the file and write to the output path
                            let mut outfile = std::fs::File::create(&outpath)?;
                            let mut buffer = Vec::new();
                            file.read_to_end(&mut buffer)?;
                            outfile.write_all(&buffer)?;
                        }
                    }

                    Ok(()) // Return Ok(()) from the closure
                })
                .await??; // Double await and double question mark: first for the task, second for the Result
            }
            ArchiveType::Gzip => {
                let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
                    std::io::Cursor::new(file_buffer),
                ));

                // Iterate over each file inside the ZIP
                for entry in archive.entries()? {
                    let mut file = entry?;
                    let outpath = Path::new(dir).join(file.path()?);

                    if file.header().entry_type().is_dir() {
                        // Create a directory if it's a folder
                        tokio::fs::create_dir_all(&outpath).await?;
                    } else {
                        // Ensure the parent folder exists
                        if let Some(parent) = outpath.parent() {
                            tokio::fs::create_dir_all(parent).await?;
                        }

                        // Extract the file and write to the output path
                        let mut outfile = tokio::fs::File::create(&outpath).await?;
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer)?;
                        outfile.write_all(&buffer).await?;
                    }
                }
            }
        }

        Ok(())
    }
}
