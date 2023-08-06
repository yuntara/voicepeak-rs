use std::path::Path;

pub use crate::errors::VoicePeakError;

pub struct VoicePeak {
    path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoicePeakNarrator {
    JapaneseFemaleChild,
    JapaneseMale1,
    JapaneseMale2,
    JapaneseMale3,
    JapaneseFemale1,
    JapaneseFemale2,
    JapaneseFemale3,
    Other(String),
}

impl std::fmt::Display for VoicePeakNarrator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JapaneseFemaleChild => write!(f, "Japanese Female Child"),
            Self::JapaneseFemale1 => write!(f, "Japanese Female 1"),
            Self::JapaneseFemale2 => write!(f, "Japanese Female 2"),
            Self::JapaneseFemale3 => write!(f, "Japanese Female 3"),
            Self::JapaneseMale1 => write!(f, "Japanese Male 1"),
            Self::JapaneseMale2 => write!(f, "Japanese Male 2"),
            Self::JapaneseMale3 => write!(f, "Japanese Male 3"),
            Self::Other(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicePeakInput {
    /// The text to speak, must be within 140 letters.
    script: String,
    narrator: VoicePeakNarrator,
    emotion: VoicePeakEmotion,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicePeakEmotion {
    happy: usize,
    sad: usize,
    angry: usize,
    fun: usize,
}

impl ToString for VoicePeakEmotion {
    fn to_string(&self) -> String {
        format!(
            "happy={},sad={},angry={},fun={}",
            self.happy, self.sad, self.angry, self.fun
        )
    }
}
impl VoicePeak {
    pub fn new(path: String) -> VoicePeak {
        VoicePeak { path: path }
    }

    pub async fn run(
        &self,
        input: VoicePeakInput,
        output_path: &Path,
    ) -> Result<(), VoicePeakError> {
        use tokio::process::Command;

        let process = Command::new(self.path.clone())
            .args(&[
                "-s",
                &input.script,
                "-n",
                &input.narrator.to_string(),
                "-o",
                output_path.to_str().unwrap(),
                "-e",
                &input.emotion.to_string(),
            ])
            .spawn()?;

        let _output = process.wait_with_output().await?;
        Ok(())
    }
}
