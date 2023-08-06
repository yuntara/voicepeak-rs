use std::path::Path;

pub use crate::errors::VoicePeakError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicePeak {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoicePeakNarrator {
    TohokuZunko,
    Zundamon,
    Other(String),
}

impl std::fmt::Display for VoicePeakNarrator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TohokuZunko => write!(f, "Tohoku Zunko"),
            Self::Zundamon => write!(f, "Zundamon"),
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
    /// 50-200
    pub speed: Option<usize>,
    // -300 - 300
    pub pitch: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum VoicePeakEmotion {
    DefaultEmotion(DefaultEmotion),
    TohokuZunkoEmotion(TohokuZunkoEmotion),
    ZundamonEmotion(ZundamonEmotion),
}

impl ToString for VoicePeakEmotion {
    fn to_string(&self) -> String {
        match self {
            Self::DefaultEmotion(emotion) => emotion.to_string(),
            Self::TohokuZunkoEmotion(emotion) => emotion.to_string(),
            Self::ZundamonEmotion(emotion) => emotion.to_string(),
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZundamonEmotion {
    pub amaama: usize,
    pub aori: usize,
    pub hisohiso: usize,
    pub live: usize,
    pub tsuntsun: usize,
}
impl ToString for ZundamonEmotion {
    fn to_string(&self) -> String {
        format!(
            "amaama={},aori={},hisohiso={},live={},tsuntsun={}",
            self.amaama, self.aori, self.hisohiso, self.live, self.tsuntsun
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
        let mut args = vec![
            "-s".to_owned(),
            input.script,
            "-n".to_owned(),
            input.narrator.to_string(),
            "-o".to_owned(),
            output_path.to_str().unwrap().to_owned(),
            "-e".to_owned(),
            input.emotion.to_string(),
        ];
        if let Some(speed) = input.speed {
            args.push("--speed".to_owned());
            args.push(speed.to_string());
        }
        if let Some(pitch) = input.pitch {
            args.push("--pitch".to_owned());
            args.push(pitch.to_string());
        }

        let process = Command::new(self.path.clone()).args(&args).spawn()?;

        let _output = process.wait_with_output().await?;
        Ok(())
    }
}
