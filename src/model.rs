use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SkillData {
    pub name: String,
    pub description: String,
    pub content: String,
    pub relative_path: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Tool,
    SystemPrompt,
    SingleTool,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tool" => Ok(Mode::Tool),
            "system_prompt" => Ok(Mode::SystemPrompt),
            "single_tool" => Ok(Mode::SingleTool),
            _ => Err(format!(
                "Invalid mode: {}. Must be 'tool', 'system_prompt', or 'single_tool'",
                s
            )),
        }
    }
}
