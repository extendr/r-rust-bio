#[derive(Debug)]
pub struct Error(pub String);

// pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rust Bio error: {}", self.0)
    }
}
