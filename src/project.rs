//! Versioned project storage. No editor or graphics dependency.
use crate::world::Scene;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub name: String,
    pub entry_scene: String,
    pub scenes: BTreeMap<String, String>,
}
pub struct ProjectStore {
    pub root: PathBuf,
    pub manifest: Manifest,
}
fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
impl ProjectStore {
    pub fn create(root: &Path, name: &str) -> Result<(Self, Scene), String> {
        if name.trim().is_empty() {
            return Err("Enter a project name".into());
        }
        // A new directory is required so existing work can never be overwritten.
        fs::create_dir(root).map_err(|e| format!("Choose a new folder: {e}"))?;
        for dir in [
            "scenes",
            "tilesets",
            "sprites",
            "animations",
            "entities",
            "dialogues",
            "audio",
            "scripts",
            "saves",
        ] {
            fs::create_dir(root.join(dir)).map_err(|e| e.to_string())?;
        }
        let mut store = Self {
            root: root.into(),
            manifest: Manifest {
                version: 1,
                name: name.trim().into(),
                entry_scene: "main".into(),
                scenes: BTreeMap::new(),
            },
        };
        let scene = Scene::default();
        store.save_scene("main", &scene)?;
        Ok((store, scene))
    }
    pub fn open(root: &Path) -> Result<(Self, Scene), String> {
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let manifest: Manifest = read_json(&root.join("project.json"))?;
        if manifest.version != 1 {
            return Err(format!(
                "Unsupported project version {}; expected 1",
                manifest.version
            ));
        }
        if manifest.name.trim().is_empty() || manifest.scenes.is_empty() {
            return Err("Project needs a name and scene".into());
        }
        let store = Self { root, manifest };
        // Validate every reference, not only the selected scene.
        for id in store.manifest.scenes.keys() {
            store.scene(id)?;
        }
        let scene = store.scene(&store.manifest.entry_scene)?;
        Ok((store, scene))
    }
    pub fn scene(&self, id: &str) -> Result<Scene, String> {
        let rel = self.manifest.scenes.get(id).ok_or("Unknown scene ID")?;
        let path = Path::new(rel);
        if path
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
        {
            return Err("Scene path must stay inside project".into());
        }
        let full = self
            .root
            .join(path)
            .canonicalize()
            .map_err(|e| e.to_string())?;
        let root = self.root.canonicalize().map_err(|e| e.to_string())?;
        if !full.starts_with(root) {
            return Err("Scene symlink escapes project".into());
        }
        let scene: Scene = read_json(&full)?;
        scene.validate()?;
        Ok(scene)
    }
    pub fn save_scene(&mut self, id: &str, scene: &Scene) -> Result<(), String> {
        scene.validate()?;
        if id.is_empty()
            || !id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err("Invalid scene ID".into());
        }
        // Immutable scene revisions + manifest-last commit keep the previous
        // complete project readable if a write is interrupted.
        let mut revision = 1;
        let rel = loop {
            let rel = format!("scenes/{id}-{revision:06}.json");
            if !self.root.join(&rel).exists() {
                break rel;
            }
            revision += 1;
        };
        write_new(
            &self.root.join(&rel),
            &serde_json::to_vec_pretty(scene).map_err(|e| e.to_string())?,
        )?;
        let mut next = self.manifest.clone();
        next.scenes.insert(id.into(), rel);
        let tmp = self.root.join("project.json.pending");
        // Stale pending files are retained for recovery; do not overwrite them.
        write_new(
            &tmp,
            &serde_json::to_vec_pretty(&next).map_err(|e| e.to_string())?,
        )?;
        fs::rename(&tmp, self.root.join("project.json")).map_err(|e| e.to_string())?;
        self.manifest = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn directory() -> PathBuf {
        std::env::temp_dir().join(format!(
            "omasprite-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }
    #[test]
    fn create_edit_save_reopen() {
        let dir = directory();
        let (mut store, mut scene) = ProjectStore::create(&dir, "Snow Village").unwrap();
        scene.tiles[34] = 3;
        scene.dialogue = "The bridge is open.".into();
        store.save_scene("main", &scene).unwrap();
        let (reopened, restored) = ProjectStore::open(&dir).unwrap();
        assert_eq!(reopened.manifest.name, "Snow Village");
        assert_eq!(restored.tiles[34], 3);
        assert_eq!(restored.dialogue, scene.dialogue);
        assert!(dir.join("scenes/main-000001.json").exists());
        assert!(ProjectStore::create(&dir, "Overwrite").is_err());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn invalid_save_leaves_previous_project_readable() {
        let dir = directory();
        let (mut store, mut scene) = ProjectStore::create(&dir, "Original").unwrap();
        scene.tiles.clear();
        assert!(store.save_scene("main", &scene).is_err());
        assert!(ProjectStore::open(&dir).is_ok());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn interrupted_manifest_keeps_previous_scene() {
        let dir = directory();
        let (mut store, mut scene) = ProjectStore::create(&dir, "Original").unwrap();
        fs::write(dir.join("project.json.pending"), b"interrupted").unwrap();
        scene.dialogue = "not committed".into();
        assert!(store.save_scene("main", &scene).is_err());
        assert_ne!(ProjectStore::open(&dir).unwrap().1.dialogue, scene.dialogue);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn future_version_and_traversal_rejected() {
        let dir = directory();
        let (store, _) = ProjectStore::create(&dir, "Original").unwrap();
        let mut m = store.manifest;
        m.version = 99;
        fs::write(dir.join("project.json"), serde_json::to_vec(&m).unwrap()).unwrap();
        assert!(ProjectStore::open(&dir).is_err());
        m.version = 1;
        m.scenes.insert("main".into(), "../escape.json".into());
        fs::write(dir.join("project.json"), serde_json::to_vec(&m).unwrap()).unwrap();
        assert!(ProjectStore::open(&dir).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
