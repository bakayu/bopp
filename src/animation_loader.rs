use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub name: String,
    pub frames: Vec<String>,
}

pub struct AnimationLoader;

impl AnimationLoader {
    pub fn load_from_file(path: &Path) -> io::Result<AnimationConfig> {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("JSON parse error: {}", e),
            )
        })
    }

    pub fn load_from_directory(dir: &Path) -> io::Result<Vec<AnimationConfig>> {
        let mut animations = Vec::new();

        if !dir.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Directory not found: {}", dir.display()),
            ));
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match Self::load_from_file(&path) {
                    Ok(config) => animations.push(config),
                    Err(e) => eprintln!("Warning: Failed to load {}: {}", path.display(), e),
                }
            }
        }

        Ok(animations)
    }

    pub fn save_to_file(config: &AnimationConfig, path: &Path) -> io::Result<()> {
        let json = serde_json::to_string_pretty(config)?;
        fs::write(path, json)?;
        Ok(())
    }
}
