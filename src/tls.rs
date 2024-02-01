use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind},
    iter,
};

extern crate rustls;
use rustls::{Certificate, PrivateKey, ServerConfig};
extern crate rustls_pemfile;
use rustls_pemfile::{read_one, Item};

use log::warn;

use crate::error;

pub struct TLSConfigBuilder {
    cert: Option<Vec<Certificate>>,
    key: Option<PrivateKey>,
}

impl TLSConfigBuilder {
    pub fn new() -> Self {
        TLSConfigBuilder {
            cert: None,
            key: None,
        }
    }

    pub fn load_certs(mut self, filename: &str) -> Result<Self, error::TlsError> {
        let certfile = File::open(filename).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to open certificate file {filename}: {e}"),
            )
        })?;
        let mut reader = BufReader::new(certfile);

        let certs: Vec<Certificate> = rustls_pemfile::certs(&mut reader)
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "rustls_pemfile failed to collect certificates from {filename}: {e}"
                ),
            )
        })?
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect();

        if certs.is_empty() {
            return Err(error::TlsError::Io(Error::new(
                ErrorKind::Other,
                format!(
                    "no certificates found in {filename}"
                ),
            )));
        }

        self.cert = Some(certs);

        Ok(self)
    }

    pub fn load_private_key(mut self, filename: &str) -> Result<Self, error::TlsError> {
        let keyfile = File::open(filename).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to open certificate file {filename}: {e}"),
            )
        })?;
        let mut reader = BufReader::new(keyfile);

        let mut keys: Vec<PrivateKey> = Vec::new();

        for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
            match item.map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!(
                        "rustls_pemfile failed to collect private key from {filename}: {e}"
                    ),
                )
            })? {
                Item::RSAKey(key) => keys.push(PrivateKey(key)),
                Item::PKCS8Key(key) => keys.push(PrivateKey(key)),
                Item::ECKey(key) => keys.push(PrivateKey(key)),
                _ => warn!("unhandled key found in {}", filename),
            }
        }

        if keys.is_empty() {
            return Err(error::TlsError::Io(Error::new(
                ErrorKind::Other,
                format!(
                    "no keys found in {filename} (encrypted keys not supported)"
                ),
            )));
        }

        if keys.len() > 1 {
            return Err(error::TlsError::Io(Error::new(
                ErrorKind::Other,
                format!(
                    "expected a single private key in {filename}"
                ),
            )));
        }

        self.key = Some(keys.first().unwrap().to_owned());

        Ok(self)
    }

    pub fn build(self) -> Result<ServerConfig, error::TlsError> {
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(self.cert.unwrap(), self.key.unwrap()).map_err(error::TlsError::Rustls)
    }
}
