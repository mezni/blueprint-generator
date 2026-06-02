use std::collections::HashMap;

pub enum ConfigSource {
    EnvVars,
    File(String),
}

impl ConfigSource {
    pub fn load(&self) -> HashMap<String, String> {
        match self {
            ConfigSource::EnvVars => std::env::vars().collect(),
            ConfigSource::File(path) => {
                let mut map = HashMap::new();
                if let Ok(content) = std::fs::read_to_string(path) {
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }
                        if let Some((key, value)) = line.split_once('=') {
                            map.insert(key.trim().to_string(), value.trim().to_string());
                        }
                    }
                }
                map
            }
        }
    }
}

impl Default for ConfigSource {
    fn default() -> Self {
        ConfigSource::EnvVars
    }
}
