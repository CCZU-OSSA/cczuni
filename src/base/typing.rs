use std::io::ErrorKind;

pub type TorErr<T> = tokio::io::Result<T>;
pub type EmptyOrErr = TorErr<()>;

#[inline]
pub fn other_error(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> tokio::io::Error {
    tokio::io::Error::new(ErrorKind::Other, error)
}
