pub type TorErr<T> = std::result::Result<T, &'static str>;
pub type EmptyOrErr = TorErr<()>;
