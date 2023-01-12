use std::error;
use std::fmt;
use std::io;
use serde_yaml;
use diesel::r2d2;

#[derive(Debug)]
pub enum StorageInitializationError {
    Migration(Box<dyn error::Error + Send + Sync + 'static>),
    R2d2(r2d2::PoolError)
}

impl error::Error for StorageInitializationError {}

impl fmt::Display for StorageInitializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Migration(ref err) => write!(f, "Failed to initialize sqlite storage (diesel migrations): {}", err),
            Self::R2d2(ref err) => write!(f, "Failed to initialize sqlite storage (r2d2 pool manager): {}", err),
        }
    }
}

impl From<Box<dyn error::Error + Send + Sync + 'static>> for StorageInitializationError {
    fn from(err: Box<dyn error::Error + Send + Sync + 'static>) -> StorageInitializationError {
        StorageInitializationError::Migration(err)
    }
}

impl From<r2d2::PoolError> for StorageInitializationError {
    fn from(err: r2d2::PoolError) -> StorageInitializationError {
        StorageInitializationError::R2d2(err)
    }
}

#[derive(Debug)]
pub enum TlsError {
    Io(io::Error),
    Rustls(rustls::Error)
}

impl error::Error for TlsError {}

impl fmt::Display for TlsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Io(ref err) => write!(f, "Encountered IO error while building tls configuration: {}", err),
            Self::Rustls(ref err) => write!(f, "Encountered Rustls error while building tls configuration: {}", err),
        }
    }
}

impl From<rustls::Error> for TlsError {
    fn from(err: rustls::Error) -> TlsError {
        TlsError::Rustls(err)
    }
}

impl From<io::Error> for TlsError {
    fn from(err: io::Error) -> TlsError {
        TlsError::Io(err)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Yaml(serde_yaml::Error),
    #[allow(dead_code)]
    DuplicatedEntry(String),
}

impl error::Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Io(ref err) => write!(f, "Encountered IO error while building deserializing configuration: {}", err),
            Self::Yaml(ref err) => write!(f, "Encountered Yaml error while building deserializing configuration: {}", err),
            Self::DuplicatedEntry(ref entry) => write!(f, "The following data is not unique in configuration: {}", entry),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}