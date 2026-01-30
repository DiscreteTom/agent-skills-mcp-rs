mod mcp;
mod model;
mod scan;

use anyhow::Result;
use clap::Parser;
use model::Mode;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "agent-skills-mcp",
    about = "Agent Skills MCP - Load Agent Skills for your agents",
    version = VERSION
)]
struct Args {
    #[arg(
        long,
        env = "SKILL_FOLDER",
        default_value = "skills",
        help = "Path to folder containing skill markdown files"
    )]
    skill_folder: String,

    #[arg(
        long,
        env = "MODE",
        default_value = "single_tool",
        help = "Operating mode"
    )]
    mode: Mode,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let expanded = shellexpand::tilde(&args.skill_folder);
    let skill_folder_path = PathBuf::from(expanded.as_ref());
    let skill_folder_path = if skill_folder_path.is_absolute() {
        skill_folder_path
    } else {
        std::env::current_dir()?.join(skill_folder_path)
    };

    let skill_folder_path = skill_folder_path
        .canonicalize()
        .unwrap_or(skill_folder_path);

    let skills = scan::scan_skills(&skill_folder_path)?;
    let server = mcp::McpServer::new(args.mode, skills, &skill_folder_path);
    server.run().await
}
