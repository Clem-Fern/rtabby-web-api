use std::io::Error;

extern crate rustls;
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    ServerConfig,
};

use crate::error;

pub struct TLSConfigBuilder;

impl TLSConfigBuilder {
    pub fn build(cert_filename: &str, key_filename: &str) -> Result<ServerConfig, error::TlsError> {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();

        let cert = CertificateDer::pem_file_iter(cert_filename)
            .map_err(|e| {
                Error::other(format!(
                    "failed to collect certificates from {cert_filename}: {e}"
                ))
            })?
            .flatten()
            .collect();

        let key = PrivateKeyDer::from_pem_file(key_filename).map_err(|e| {
            Error::other(format!(
                "failed to collect private key from {key_filename}: {e}"
            ))
        })?;

        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert, key)
            .map_err(error::TlsError::Rustls)
    }
}
