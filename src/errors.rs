use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoicePeakError {
    SpawnError(#[from] tokio::io::Error),
}

impl std::fmt::Display for VoicePeakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoicePeakError::SpawnError(e) => write!(f, "SpawnError: {}", e),
        }
    }
}
