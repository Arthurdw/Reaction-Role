use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs::File, io::Read};

fn wrap_error<T>(result: Result<T, std::io::Error>, path: &str, action: &str) -> Result<T> {
    result.map_err(|e| anyhow::anyhow!("Failed to load {}: Error {}: {}", path, action, e))
}

pub(crate) fn load_yaml_config<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T> {
    let mut file = wrap_error(File::open(path), path, "opening file")?;
    let mut contents = String::new();
    wrap_error(file.read_to_string(&mut contents), path, "reading file")?;
    let config: T = serde_yaml::from_str(&contents)
        .with_context(|| format!("Failed to load {}: Error parsing YAML", path))?;

    Ok(config)
}
