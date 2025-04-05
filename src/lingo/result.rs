#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("input is empty")]
    EmptyInput,
}

pub type Result<T> = std::result::Result<T, Error>;
