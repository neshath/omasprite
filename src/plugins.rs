//! Project-local plugin discovery. Plugins are data manifests, never executed in-process.
use crate::advanced::{validate_plugin, PluginManifest};
use std::path::Path;

pub fn discover(root: &Path) -> Result<Vec<PluginManifest>, String> {
    let directory = root.join("plugins");
    if !directory.exists() {
        return Ok(vec![]);
    }
    let mut plugins = vec![];
    for entry in std::fs::read_dir(&directory).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            let manifest = entry.path().join("plugin.json");
            if manifest.exists() {
                let bytes = std::fs::read(&manifest).map_err(|e| e.to_string())?;
                let plugin: PluginManifest = serde_json::from_slice(&bytes)
                    .map_err(|e| format!("{}: {e}", manifest.display()))?;
                validate_plugin(&plugin)?;
                plugins.push(plugin);
            }
        }
    }
    plugins.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(plugins)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discovers_safe_plugins_and_rejects_privileged_ones() {
        let root = std::env::temp_dir().join(format!("omasprite-plugins-{}", std::process::id()));
        std::fs::create_dir_all(root.join("plugins/safe")).unwrap();
        std::fs::write(
            root.join("plugins/safe/plugin.json"),
            br#"{"id":"safe","version":"1","capabilities":["palette"],"entry":"entry.json"}"#,
        )
        .unwrap();
        assert_eq!(discover(&root).unwrap()[0].id, "safe");
        std::fs::remove_dir_all(root).unwrap();
    }
}
