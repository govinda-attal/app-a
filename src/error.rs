#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("generic error {0}")]
    Generic(String),

    #[error("unimplemented: {0}")]
    Unimplemented(String),

    #[error("bad request: {0}")]
    BadRquest(String),

    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error(transparent)]
    UuidParseError(#[from] uuid::Error),

    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),

    #[error(transparent)]
    TonicReflectionError(#[from] tonic_reflection::server::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    SqlxMigrateError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    TaskJoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    OneShotRecvError(#[from] tokio::sync::oneshot::error::RecvError),
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        use tonic::{Code, Status};
        match e {
            Error::BadRquest(str) => Status::new(Code::InvalidArgument, str),
            Error::Generic(str) => Status::new(Code::Internal, str),
            Error::Unimplemented(str) => Status::new(Code::Unimplemented, str),
            _ => Status::new(Code::Internal, e.to_string()),
        }
    }
}
