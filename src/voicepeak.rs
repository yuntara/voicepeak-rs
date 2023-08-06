use std::path::Path;

pub use crate::errors::VoicePeakError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicePeak {
    pub path: String,
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
    TohokuZunko,
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
            Self::TohokuZunko => write!(f, "Tohoku Zunko"),
            Self::Other(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicePeakInput {
    /// The text to speak, must be within 140 letters.
    pub script: String,
    pub narrator: VoicePeakNarrator,
    pub emotion: VoicePeakEmotion,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum VoicePeakEmotion {
    DefaultEmotion(DefaultEmotion),
    TohokuZunkoEmotion(TohokuZunkoEmotion),
}

impl ToString for VoicePeakEmotion {
    fn to_string(&self) -> String {
        match self {
            Self::DefaultEmotion(emotion) => emotion.to_string(),
            Self::TohokuZunkoEmotion(emotion) => emotion.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefaultEmotion {
    pub happy: usize,
    pub sad: usize,
    pub angry: usize,
    pub fun: usize,
}

impl ToString for DefaultEmotion {
    fn to_string(&self) -> String {
        format!(
            "happy={},sad={},angry={},fun={}",
            self.happy, self.sad, self.angry, self.fun
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TohokuZunkoEmotion {
    pub sad: usize,
    pub astonished: usize,
    pub firm: usize,
    pub live: usize,
    pub soft: usize,
}
impl ToString for TohokuZunkoEmotion {
    fn to_string(&self) -> String {
        format!(
            "sad={},astonished={},firm={},live={},soft={}",
            self.sad, self.astonished, self.firm, self.live, self.soft
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
