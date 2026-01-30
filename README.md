# agent-skills-mcp - Load [Agent Skills](https://agentskills.io/home) for your agents

[![npm version](https://img.shields.io/npm/v/agent-skills-mcp)](https://www.npmjs.com/package/agent-skills-mcp)
[![GitHub release](https://img.shields.io/github/v/release/DiscreteTom/agent-skills-mcp-rs)](https://github.com/DiscreteTom/agent-skills-mcp-rs/releases)

## Usage

### Full CLI Usage

<details>

<summary><code>agent-skills-mcp --help</code></summary>

```sh
Agent Skills MCP - Load Agent Skills for your agents

Usage: agent-skills-mcp [OPTIONS]

Options:
      --skill-folder <SKILL_FOLDER>  Path to folder containing skill markdown files [env: SKILL_FOLDER=] [default: skills]
      --mode <MODE>                  Operating mode [env: MODE=] [default: tool]
  -h, --help                         Print help
  -V, --version                      Print version
```

</details>

### Setup

First, put your skills in `~/skills`, e.g.

```sh
git clone https://github.com/anthropics/skills.git ~/skills
```

The server recursively searches for `SKILL.md` files and follows symlinks, allowing flexible skill organization.

Then, add this to your MCP client configuration:

[![Install MCP Server](https://cursor.com/deeplink/mcp-install-dark.svg)](https://cursor.com/cn/install-mcp?name=skills&config=eyJlbnYiOnsiU0tJTExfRk9MREVSIjoifi9za2lsbHMifSwiY29tbWFuZCI6Im5weCAteSBhZ2VudC1za2lsbHMtbWNwIn0%3D)
[![Add to Kiro](https://kiro.dev/images/add-to-kiro.svg)](https://kiro.dev/launch/mcp/add?name=skills&config=%7B%22command%22%3A%22npx%22%2C%22args%22%3A%5B%22-y%22%2C%22agent-skills-mcp%22%5D%2C%22env%22%3A%7B%22SKILL_FOLDER%22%3A%22~%2Fskills%22%7D%7D)

```json
{
  "mcpServers": {
    "skills": {
      "command": "npx",
      "args": ["-y", "agent-skills-mcp"],
      "env": {
        "SKILL_FOLDER": "~/skills"
      }
    }
  }
}
```

### Modes

- `single_tool` (default): Register a single `get_skill` tool that accepts a skill name parameter, with all skills listed in the tool description. This reduces the number of tools exposed to the agent.
- `tool`: Register each skill as a separate MCP tool (e.g., `get_skill_name1`, `get_skill_name2`)
- `system_prompt`: Include skill information in MCP instructions

## Install Standalone Binary

<details>

<summary>Shell (macOS/Linux)</summary>

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/DiscreteTom/agent-skills-mcp-rs/releases/latest/download/agent-skills-mcp-installer.sh | sh
```

</details>

<details>

<summary>PowerShell (Windows)</summary>

```powershell
irm https://github.com/DiscreteTom/agent-skills-mcp-rs/releases/latest/download/agent-skills-mcp-installer.ps1 | iex
```

</details>

<details>

<summary>npm</summary>

```sh
npm install -g agent-skills-mcp
```

</details>

## [CHANGELOG](./CHANGELOG.md)
