# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-30

### Added
- Initial Rust implementation of agent-skills-mcp
- Support for scanning SKILL.md files recursively with symlink support
- YAML frontmatter parsing for skill metadata (name, description)
- Two operating modes: tool and system_prompt
- MCP protocol implementation with JSON-RPC over stdio
- CLI with environment variable support (SKILL_FOLDER, MODE)
- Identical functionality to Python version

[unreleased]: https://github.com/DiscreteTom/agent-skills-mcp-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/DiscreteTom/agent-skills-mcp-rs/releases/tag/v0.1.0
