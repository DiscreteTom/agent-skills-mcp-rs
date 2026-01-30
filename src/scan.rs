use crate::model::SkillData;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

fn parse_markdown_file(md_file: &Path, folder: &Path, content: &str) -> Result<SkillData> {
    let parsed = gray_matter::Matter::<gray_matter::engine::YAML>::new().parse(content);

    let default_name = md_file.file_stem().unwrap().to_str().unwrap().to_string();

    let (name, description) = if let Some(data) = &parsed.data {
        match data.as_hashmap() {
            Ok(hashmap) => {
                let name = hashmap
                    .get("name")
                    .and_then(|v| v.as_string().ok())
                    .unwrap_or(default_name.clone());
                let description = hashmap
                    .get("description")
                    .and_then(|v| v.as_string().ok())
                    .unwrap_or_default();
                (name, description)
            }
            Err(_) => (default_name, String::new()),
        }
    } else {
        (default_name, String::new())
    };

    let relative_path = md_file.strip_prefix(folder)?.to_path_buf();

    Ok(SkillData {
        name,
        description,
        content: parsed.content,
        relative_path,
    })
}

pub fn scan_skills(folder: &Path) -> Result<Vec<SkillData>> {
    if !folder.exists() || !folder.is_dir() {
        eprintln!(
            "Warning: folder path '{}' does not exist or is not a directory",
            folder.display()
        );
        return Ok(Vec::new());
    }

    let mut skills = Vec::new();

    for entry in WalkDir::new(folder).follow_links(true) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Warning: failed to read entry: {}", e);
                continue;
            }
        };

        if entry.file_type().is_file() && entry.file_name() == "SKILL.md" {
            match std::fs::read_to_string(entry.path()) {
                Ok(content) => match parse_markdown_file(entry.path(), folder, &content) {
                    Ok(skill) => skills.push(skill),
                    Err(e) => {
                        eprintln!("Warning: failed to parse {}: {}", entry.path().display(), e)
                    }
                },
                Err(e) => eprintln!("Warning: failed to read {}: {}", entry.path().display(), e),
            }
        }
    }

    Ok(skills)
}
