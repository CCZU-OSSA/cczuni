pub type TorErr<T> = tokio::io::Result<T>;
pub type EmptyOrErr = TorErr<()>;
