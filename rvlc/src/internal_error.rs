use thiserror::Error;

pub type VLCResult<I> = Result<I, VLCError>;

#[derive(Debug, Error)]
pub enum VLCError {
    #[error("{0} returned a null pointer")]
    NullPointer(String),
    #[error("Failed to marshal string to FFI string")]
    StrFFI,
    #[error("Failed to marshal path to FFI path")]
    PathFFI,
    #[error("Unknown libvlc error")]
    Unknown,
}
