use std::io::ErrorKind;

pub type TorErr<T> = tokio::io::Result<T>;
pub type EmptyOrErr = TorErr<()>;

#[inline]
pub fn convert_error(error: impl ToString) -> tokio::io::Error {
    tokio::io::Error::new(ErrorKind::Other, error.to_string())
}
