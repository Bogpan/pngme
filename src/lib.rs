use std::fmt::Display;

pub mod chunk;
pub mod chunk_type;
pub mod png;

pub type Error = Box<dyn std::error::Error>;

#[derive(Debug, Clone, Copy)]
pub enum PngError {
    InvalidLength,
    InvalidHeader,
}

#[derive(Debug, Clone, Copy)]
pub enum ChunkError {
    InvalidBytes,
    InvalidCrc,
    UnknownChunkType,
}

impl std::error::Error for ChunkError {}
impl std::error::Error for PngError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            ChunkError::InvalidBytes => "Bytes are outside the ASCII alphabetic range",
            ChunkError::InvalidCrc => "Given CRC is incorrect",
            ChunkError::UnknownChunkType => "Unknown chunk type",
        };

        write!(f, "{fmt}")
    }
}

impl Display for PngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            PngError::InvalidLength => {
                "The file doesn't have the necessary length to be a PNG file"
            }
            PngError::InvalidHeader => "The header isn't the valid PNG header",
        };

        write!(f, "{fmt}")
    }
}
