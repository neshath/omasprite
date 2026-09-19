//! Deterministic game state; no windowing or rendering dependencies.
use crate::world::Scene;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Portal {
    pub at: [usize; 2],
    pub map: String,
    pub spawn: [usize; 2],
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Save {
    pub version: u32,
    pub map: String,
    pub position: [usize; 2],
    pub completed: bool,
}
pub struct Runtime {
    pub maps: BTreeMap<String, Scene>,
    pub state: Save,
    pub page: Option<usize>,
    pub direction: [i32; 2],
    pub log: Vec<String>,
}
impl Runtime {
    fn npc_at(scene: &Scene, position: [usize; 2]) -> bool {
        scene.npcs.iter().any(|n| *n == position)
            || (scene.npcs.is_empty() && scene.npc == position)
    }
    pub fn new(maps: BTreeMap<String, Scene>, entry: &str) -> Result<Self, String> {
        for scene in maps.values() {
            scene.validate()?;
            for p in &scene.portals {
                let dest = maps.get(&p.map).ok_or("Portal destination map missing")?;
                if !dest.walkable(p.spawn[0], p.spawn[1]) || Self::npc_at(dest, p.spawn) {
                    return Err("Portal destination is blocked".into());
                }
            }
        }
        let scene = maps.get(entry).ok_or("Entry map missing")?;
        if !scene.walkable(scene.spawn[0], scene.spawn[1]) || Self::npc_at(scene, scene.spawn) {
            return Err("Player spawn is blocked".into());
        }
        let position = scene.spawn;
        Ok(Self {
            maps,
            state: Save {
                version: 1,
                map: entry.into(),
                position,
                completed: false,
            },
            page: None,
            direction: [0, 1],
            log: vec!["Game started".into()],
        })
    }
    pub fn scene(&self) -> &Scene {
        &self.maps[&self.state.map]
    }
    pub fn step(&mut self, dx: i32, dy: i32) -> bool {
        if self.page.is_some() || dx.abs() + dy.abs() != 1 {
            return false;
        }
        self.direction = [dx, dy];
        let [x, y] = self.state.position;
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 {
            return false;
        }
        let next = [nx as usize, ny as usize];
        let s = self.scene();
        if !s.walkable(next[0], next[1])
            || Self::npc_at(s, next)
            || s.heights[next[1] * 16 + next[0]].abs_diff(s.heights[y * 16 + x]) > 1
        {
            return false;
        }
        let portal = s.portals.iter().find(|p| p.at == next).cloned();
        self.state.position = next;
        if let Some(p) = portal {
            self.state.map = p.map;
            self.state.position = p.spawn;
            self.log.push(format!("Entered {}", self.state.map));
        }
        if self.scene().objective == Some(self.state.position) {
            self.state.completed = true;
            self.log.push("Objective completed".into());
        }
        if self.log.len() > 64 {
            self.log.remove(0);
        }
        true
    }
    pub fn interact(&mut self) {
        if let Some(page) = self.page {
            self.page = if page + 1 < self.scene().dialogue.split('|').count() {
                Some(page + 1)
            } else {
                None
            };
            return;
        }
        let p = self.state.position;
        if self
            .scene()
            .npcs
            .iter()
            .chain(std::iter::once(&self.scene().npc))
            .any(|n| p[0].abs_diff(n[0]) + p[1].abs_diff(n[1]) == 1)
        {
            self.page = Some(0);
        }
    }
    pub fn line(&self) -> Option<&str> {
        self.page
            .and_then(|i| self.scene().dialogue.split('|').nth(i))
    }
    pub fn restore(&mut self, save: Save) -> Result<(), String> {
        let scene = self
            .maps
            .get(&save.map)
            .ok_or("Save references unknown map")?;
        if save.version != 1
            || !scene.walkable(save.position[0], save.position[1])
            || Self::npc_at(scene, save.position)
        {
            return Err("Invalid game save position/version".into());
        }
        self.state = save;
        self.page = None;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_example_complete_route() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/snow-courier");
        let (store, _) = crate::project::ProjectStore::open(&root).unwrap();
        let maps: BTreeMap<_, _> = store
            .manifest
            .scenes
            .keys()
            .map(|id| (id.clone(), store.scene(id).unwrap()))
            .collect();
        let mut r = Runtime::new(maps.clone(), "main").unwrap();
        assert!(r.step(0, -1));
        r.interact();
        assert!(r.line().is_some());
        r.interact();
        assert!(r.line().is_some());
        r.interact();
        assert!(r.line().is_none());
        // Visit the lodge and leave through its data-authored portal.
        for _ in 0..3 {
            assert!(r.step(-1, 0));
        }
        assert!(r.step(0, -1));
        assert_eq!(r.state.map, "interior");
        assert!(r.step(0, 1));
        assert_eq!(r.state.map, "main");
        assert!(r.step(0, 1));
        for _ in 0..9 {
            assert!(r.step(1, 0));
        }
        assert_eq!(r.state.map, "path");
        for _ in 0..12 {
            assert!(r.step(1, 0));
        }
        assert!(r.state.completed);
        let encoded = serde_json::to_vec(&r.state).unwrap();
        let mut reopened = Runtime::new(maps, "main").unwrap();
        reopened
            .restore(serde_json::from_slice(&encoded).unwrap())
            .unwrap();
        assert_eq!(reopened.state, r.state);
    }
    #[test]
    fn dialogue_blocks_movement_and_advances() {
        let s = Scene {
            spawn: [8, 8],
            dialogue: "Hello|Goodbye".into(),
            ..Default::default()
        };
        let mut r = Runtime::new(BTreeMap::from([("main".into(), s)]), "main").unwrap();
        r.interact();
        assert_eq!(r.line(), Some("Hello"));
        assert!(!r.step(1, 0));
        r.interact();
        assert_eq!(r.line(), Some("Goodbye"));
        r.interact();
        assert!(r.step(1, 0));
    }
    #[test]
    fn transition_objective_save_restore() {
        let mut a = Scene::default();
        a.portals.push(Portal {
            at: [9, 9],
            map: "path".into(),
            spawn: [2, 2],
        });
        let b = Scene {
            objective: Some([3, 2]),
            ..Default::default()
        };
        let maps = BTreeMap::from([("main".into(), a), ("path".into(), b)]);
        let mut r = Runtime::new(maps.clone(), "main").unwrap();
        assert!(r.step(1, 0));
        assert_eq!(r.state.map, "path");
        assert!(r.step(1, 0));
        assert!(r.state.completed);
        let save: Save = serde_json::from_str(&serde_json::to_string(&r.state).unwrap()).unwrap();
        let mut fresh = Runtime::new(maps, "main").unwrap();
        fresh.restore(save).unwrap();
        assert_eq!(fresh.state, r.state);
    }
    #[test]
    fn collision_and_boundary() {
        let mut s = Scene {
            spawn: [0, 0],
            ..Default::default()
        };
        s.tiles[1] = 4;
        let mut r = Runtime::new(BTreeMap::from([("main".into(), s)]), "main").unwrap();
        assert!(!r.step(-1, 0));
        assert!(!r.step(1, 0));
        assert!(r.step(0, 1));
    }

    #[test]
    fn authored_collision_layer_blocks_walkable_tile() {
        let mut s = Scene::default();
        s.collision[9 * 16 + 9] = true;
        let mut r = Runtime::new(BTreeMap::from([("main".into(), s)]), "main").unwrap();
        assert!(!r.step(1, 0));
    }

    #[test]
    fn scene_light_roundtrips_and_validates_radius() {
        let mut scene = Scene::default();
        scene.light = [3, 4];
        scene.light_radius = 12.0;
        let copy: Scene = serde_json::from_slice(&serde_json::to_vec(&scene).unwrap()).unwrap();
        assert_eq!(copy.light, [3, 4]);
        assert!(copy.validate().is_ok());
        scene.light_radius = 0.0;
        assert!(scene.validate().is_err());
    }

    #[test]
    fn multiple_npcs_are_blocking_and_interactable() {
        let mut scene = Scene::default();
        scene.npcs = vec![[8, 7], [10, 9]];
        scene.npc = [8, 7];
        let mut runtime = Runtime::new(BTreeMap::from([("main".into(), scene)]), "main").unwrap();
        assert!(runtime.step(0, -1));
        assert!(runtime.step(1, 0));
        assert!(runtime.step(0, 1));
        assert!(!runtime.step(1, 0));
        runtime.interact();
        assert!(runtime.line().is_some());
    }
}
