#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] twelf::Error),

    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),

    #[error(transparent)]
    TonicReflectionError(#[from] tonic_reflection::server::Error),
}
