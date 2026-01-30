use crate::model::{Mode, SkillData};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Deserialize)]
struct Request {
    #[serde(default)]
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize)]
struct Response {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ErrorObject>,
}

#[derive(Serialize)]
struct ErrorObject {
    code: i32,
    message: String,
}

pub struct McpServer {
    mode: Mode,
    skills: Vec<SkillData>,
    skill_folder: String,
}

impl McpServer {
    pub fn new(mode: Mode, skills: Vec<SkillData>, skill_folder: &Path) -> Self {
        Self {
            mode,
            skills,
            skill_folder: skill_folder.display().to_string(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            if let Ok(req) = serde_json::from_str::<Request>(&line) {
                if let Some(resp) = self.handle_request(req) {
                    let json = serde_json::to_string(&resp)?;
                    stdout.write_all(json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
            }
            line.clear();
        }
        Ok(())
    }

    fn handle_request(&self, req: Request) -> Option<Response> {
        match req.method.as_str() {
            "initialize" => Some(self.handle_initialize(req.id)),
            "notifications/initialized" => None,
            "tools/list" => Some(self.handle_tools_list(req.id)),
            "tools/call" => Some(self.handle_tools_call(req)),
            _ => Some(Response {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(ErrorObject {
                    code: -32601,
                    message: "Method not found".to_string(),
                }),
            }),
        }
    }

    fn handle_initialize(&self, id: Option<Value>) -> Response {
        let instructions = match self.mode {
            Mode::SystemPrompt => self.build_system_prompt_instructions(),
            _ => String::new(),
        };

        Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2025-06-18",
                "capabilities": { "tools": {} },
                "serverInfo": {
                    "name": "agent-skills-mcp",
                    "version": "0.1.0"
                },
                "instructions": instructions
            })),
            error: None,
        }
    }

    fn handle_tools_list(&self, id: Option<Value>) -> Response {
        match self.mode {
            Mode::Tool => {
                let tools: Vec<Value> = self
                    .skills
                    .iter()
                    .map(|skill| {
                        json!({
                            "name": format!("get_skill_{}", skill.name),
                            "description": format!(
                                "Returns the content of the skill file at: {}/{}\n\n## Skill Description\n{}",
                                self.skill_folder, skill.relative_path.display(), skill.description
                            ),
                            "inputSchema": {
                                "type": "object",
                                "properties": {},
                                "required": []
                            }
                        })
                    })
                    .collect();

                Response {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(json!({ "tools": tools })),
                    error: None,
                }
            }
            Mode::SingleTool => {
                let description = self.build_single_tool_description();
                let tools = vec![json!({
                    "name": "get_skill",
                    "description": description,
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "The name of the skill to retrieve"
                            }
                        },
                        "required": ["name"]
                    }
                })];

                Response {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(json!({ "tools": tools })),
                    error: None,
                }
            }
            Mode::SystemPrompt => Response {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({ "tools": [] })),
                error: None,
            },
        }
    }

    fn handle_tools_call(&self, req: Request) -> Response {
        let name = req
            .params
            .as_ref()
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str());

        if let Some(name) = name {
            if name == "get_skill" && matches!(self.mode, Mode::SingleTool) {
                let skill_name = req
                    .params
                    .as_ref()
                    .and_then(|p| p.get("arguments"))
                    .and_then(|a| a.get("name"))
                    .and_then(|n| n.as_str());

                if let Some(skill_name) = skill_name {
                    if let Some(skill) = self.skills.iter().find(|s| s.name == skill_name) {
                        return Response {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: Some(json!({
                                "content": [{
                                    "type": "text",
                                    "text": skill.content
                                }]
                            })),
                            error: None,
                        };
                    }
                }
            } else if let Some(skill_name) = name.strip_prefix("get_skill_") {
                if let Some(skill) = self.skills.iter().find(|s| s.name == skill_name) {
                    return Response {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(json!({
                            "content": [{
                                "type": "text",
                                "text": skill.content
                            }]
                        })),
                        error: None,
                    };
                }
            }
        }

        Response {
            jsonrpc: "2.0".to_string(),
            id: req.id,
            result: None,
            error: Some(ErrorObject {
                code: -32602,
                message: "Tool not found".to_string(),
            }),
        }
    }

    fn build_system_prompt_instructions(&self) -> String {
        let mut instructions = String::from(
            "\nThis MCP server is just a loader of skills. \n\
            The loading is completed.\n\n\
            Here are the discovered skills and their brief description. \n\
            Read the corresponding SKILL.md file to get familiar with their details:\n\n",
        );

        for skill in &self.skills {
            instructions.push_str(&format!(
                "\n## {}\n\n> Path: {}/{}\n\n{}\n\n",
                skill.name,
                self.skill_folder,
                skill.relative_path.display(),
                skill.description
            ));
        }

        instructions
    }

    fn build_single_tool_description(&self) -> String {
        let mut description = String::from(
            "Get the content of a skill by name.\n\n\
            Available skills:\n\n",
        );

        for skill in &self.skills {
            description.push_str(&format!(
                "## {}\n\n> Path: {}/{}\n\n{}\n\n",
                skill.name,
                self.skill_folder,
                skill.relative_path.display(),
                skill.description
            ));
        }

        description
    }
}
