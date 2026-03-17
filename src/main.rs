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
        value_delimiter = ',',
        default_value = "skills",
        help = "Path(s) to folder(s) containing skill markdown files (repeat or comma-separate)"
    )]
    skill_folder: Vec<String>,

    #[arg(
        long,
        env = "MODE",
        default_value = "single_tool",
        help = "Operating mode"
    )]
    mode: Mode,
}

fn resolve_folder(raw: &str) -> PathBuf {
    let expanded = shellexpand::tilde(raw);
    let path = PathBuf::from(expanded.as_ref());
    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .expect("failed to get current dir")
            .join(path)
    };
    path.canonicalize().unwrap_or(path)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let skills: Vec<_> = args
        .skill_folder
        .iter()
        .flat_map(|raw| {
            let path = resolve_folder(raw);
            scan::scan_skills(&path).unwrap_or_default()
        })
        .collect();

    let server = mcp::McpServer::new(args.mode, skills);
    server.run().await
}
