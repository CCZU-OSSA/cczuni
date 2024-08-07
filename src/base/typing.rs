pub type Result<T> = std::result::Result<T, &'static str>;
pub type EmptyOrErr = Result<()>;
