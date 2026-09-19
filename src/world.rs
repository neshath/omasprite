//! Editable height-field scene and small native RPG runtime.
use egui::{self, Color32, Rect, Sense, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    #[serde(default)]
    pub character: crate::advanced::CharacterSet,
    #[serde(default)]
    pub sprite: crate::sprite::Sprite,
    #[serde(default)]
    pub portals: Vec<crate::runtime::Portal>,
    #[serde(default)]
    pub objective: Option<[usize; 2]>,
    #[serde(default = "default_light")]
    pub light: [usize; 2],
    #[serde(default = "default_light_radius")]
    pub light_radius: f32,
    pub version: u32,
    pub tiles: Vec<u8>,
    #[serde(default = "default_collision")]
    pub collision: Vec<bool>,
    pub heights: Vec<u8>,
    pub npc: [usize; 2],
    #[serde(default)]
    pub npcs: Vec<[usize; 2]>,
    pub spawn: [usize; 2],
    pub dialogue: String,
    #[serde(default)]
    pub dialogue_nodes: Vec<crate::advanced::DialogueNode>,
    pub ambient: f32,
}
fn default_collision() -> Vec<bool> {
    vec![false; 256]
}
fn default_light() -> [usize; 2] {
    [8, 8]
}
fn default_light_radius() -> f32 {
    8.0
}
impl Default for Scene {
    fn default() -> Self {
        Self {
            character: crate::advanced::CharacterSet::default(),
            sprite: crate::sprite::Sprite::default(),
            portals: vec![],
            objective: None,
            light: default_light(),
            light_radius: default_light_radius(),
            version: 1,
            tiles: vec![0; 256],
            collision: default_collision(),
            heights: vec![0; 256],
            npc: [8, 7],
            npcs: vec![[8, 7]],
            spawn: [8, 9],
            dialogue: "Welcome! Build your village and tell its story.".into(),
            dialogue_nodes: vec![],
            ambient: 0.85,
        }
    }
}
impl Scene {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1
            || !self.sprite.validate()
            || self
                .portals
                .iter()
                .any(|p| p.at.iter().chain(p.spawn.iter()).any(|v| *v >= 16) || p.map.is_empty())
            || self.objective.is_some_and(|p| p.iter().any(|v| *v >= 16))
            || self.light.iter().any(|v| *v >= 16)
            || !self.light_radius.is_finite()
            || !(1.0..=32.0).contains(&self.light_radius)
            || self.tiles.len() != 256
            || self.collision.len() != 256
            || self.heights.len() != 256
            || self.tiles.iter().any(|v| *v > 4)
            || self.heights.iter().any(|v| *v > 4)
            || self.npc.iter().chain(self.spawn.iter()).any(|v| *v >= 16)
            || self.npcs.iter().any(|n| n.iter().any(|v| *v >= 16))
            || !self.ambient.is_finite()
            || crate::advanced::validate_dialogue(&self.dialogue_nodes).is_err()
            || !(0.2..=1.0).contains(&self.ambient)
        {
            return Err("Invalid scene data".into());
        }
        Ok(())
    }
    pub fn walkable(&self, x: usize, y: usize) -> bool {
        x < 16 && y < 16 && self.tiles[y * 16 + x] < 2 && !self.collision[y * 16 + x]
    }
}
pub struct World {
    sprite_editor: crate::sprite::Editor,
    runtime: Option<crate::runtime::Runtime>,
    active_map: String,
    new_map: String,
    portal_target: String,
    portal_spawn: [usize; 2],
    project_store: Option<crate::project::ProjectStore>,
    project_folder: String,
    project_name: String,
    saved_scene: Option<Scene>,
    discard_confirmed: bool,
    pub scene: Scene,
    pub path: String,
    pub status: String,
    brush: u8,
    elevation: u8,
    player: [usize; 2],
    talking: bool,
    dialogue_choice: usize,
    history: Vec<Scene>,
    camera: crate::tilemap::Camera,
    particles: crate::particles::System,
    weather: crate::advanced::ParticleEmitter,
}
impl Default for World {
    fn default() -> Self {
        Self {
            sprite_editor: crate::sprite::Editor::default(),
            runtime: None,
            active_map: "main".into(),
            new_map: "new-map".into(),
            portal_target: "main".into(),
            portal_spawn: [8, 9],
            project_store: None,
            project_folder: std::env::var_os("HOME")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(std::env::temp_dir)
                .join("Documents")
                .join("my-omasprite-game")
                .display()
                .to_string(),
            project_name: "Untitled".into(),
            saved_scene: None,
            discard_confirmed: false,
            scene: Scene::default(),
            path: "scene.omasprite.json".into(),
            status: "New unsaved scene".into(),
            brush: 0,
            elevation: 0,
            player: [8, 9],
            talking: false,
            dialogue_choice: 0,
            history: vec![],
            camera: crate::tilemap::Camera::new([16.0, 12.0]),
            particles: crate::particles::System::default(),
            weather: crate::advanced::ParticleEmitter {
                name: "snow".into(),
                rate: 8.0,
                lifetime: 8.0,
                color: [235, 243, 248, 180],
                wind: [0.15, 0.0],
            },
        }
    }
}
impl World {
    pub fn sprite_ui(&mut self, ui: &mut egui::Ui) {
        self.sprite_editor.show(ui, &mut self.scene.sprite);
    }
    pub fn project_label(&self) -> String {
        let name = self
            .project_store
            .as_ref()
            .map(|s| s.manifest.name.as_str())
            .unwrap_or("Untitled");
        format!(
            "{} · {}",
            name,
            if self.saved_scene.as_ref() == Some(&self.scene) {
                "saved"
            } else {
                "unsaved"
            }
        )
    }
    fn project_action(&mut self, create: bool) {
        if self.saved_scene.as_ref() != Some(&self.scene) && !self.discard_confirmed {
            self.status =
                "Unsaved scene: save it or check Discard unsaved edits before switching.".into();
            return;
        }
        let root = std::path::Path::new(&self.project_folder);
        let result = if create {
            crate::project::ProjectStore::create(root, &self.project_name)
        } else {
            crate::project::ProjectStore::open(root)
        };
        match result {
            Ok((store, scene)) => {
                self.active_map = store.manifest.entry_scene.clone();
                self.scene = scene;
                self.saved_scene = Some(self.scene.clone());
                self.project_store = Some(store);
                self.history.clear();
                self.start();
                self.discard_confirmed = false;
                self.status = "Project ready".into();
            }
            Err(e) => self.status = e,
        }
    }
    pub fn from_file(path: String) -> Self {
        if std::path::Path::new(&path).is_dir() {
            let mut world = Self {
                project_folder: path,
                discard_confirmed: true,
                ..Default::default()
            };
            world.project_action(false);
            return world;
        }
        let mut world = Self {
            path,
            ..Default::default()
        };
        world.load();
        world
    }
    pub fn start(&mut self) {
        self.runtime = None;
        self.player = self.scene.spawn;
        self.talking = false;
        let mut maps = std::collections::BTreeMap::new();
        if let Some(store) = &self.project_store {
            for id in store.manifest.scenes.keys() {
                match store.scene(id) {
                    Ok(s) => {
                        maps.insert(id.clone(), s);
                    }
                    Err(e) => {
                        self.status = e;
                        return;
                    }
                }
            }
        }
        maps.insert(self.active_map.clone(), self.scene.clone());
        match crate::runtime::Runtime::new(maps, &self.active_map) {
            Ok(r) => {
                self.runtime = Some(r);
            }
            Err(e) => {
                self.runtime = None;
                self.status = e;
            }
        }
    }
    pub fn load(&mut self) {
        let result = std::fs::read(&self.path)
            .map_err(|e| e.to_string())
            .and_then(|b| serde_json::from_slice::<Scene>(&b).map_err(|e| e.to_string()))
            .and_then(|s| {
                s.validate()?;
                Ok(s)
            });
        match result {
            Ok(s) => {
                self.scene = s;
                self.saved_scene = Some(self.scene.clone());
                self.history.clear();
                self.start();
                self.status = "Scene loaded".into();
            }
            Err(e) => self.status = e,
        }
    }
    pub fn save(&mut self) {
        if let Some(store) = self.project_store.as_mut() {
            let id = self.active_map.clone();
            match store.save_scene(&id, &self.scene) {
                Ok(()) => {
                    self.saved_scene = Some(self.scene.clone());
                    self.status = "Project saved".into();
                }
                Err(e) => self.status = e,
            }
            return;
        }
        let result = self
            .scene
            .validate()
            .and_then(|_| serde_json::to_vec_pretty(&self.scene).map_err(|e| e.to_string()))
            .and_then(|b| {
                let tmp = format!("{}.tmp", self.path);
                std::fs::write(&tmp, b)
                    .and_then(|_| std::fs::rename(&tmp, &self.path))
                    .map_err(|e| e.to_string())
            });
        self.status = match result {
            Ok(()) => {
                self.saved_scene = Some(self.scene.clone());
                "Scene saved. Share this JSON with another Omasprite installation.".into()
            }
            Err(e) => e,
        };
    }
    pub fn show(&mut self, ui: &mut egui::Ui, playing: bool) {
        if !playing {
            if ui.button("Undo map edit").clicked() {
                if let Some(scene) = self.history.pop() {
                    self.scene = scene;
                }
            }
            let ids = self
                .project_store
                .as_ref()
                .map(|s| s.manifest.scenes.keys().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            ui.horizontal_wrapped(|ui| {
                for id in ids {
                    if ui.selectable_label(self.active_map == id, &id).clicked() {
                        self.save();
                        if self.saved_scene.as_ref() == Some(&self.scene) {
                            if let Some(store) = &self.project_store {
                                match store.scene(&id) {
                                    Ok(scene) => {
                                        self.scene = scene;
                                        self.saved_scene = Some(self.scene.clone());
                                        self.active_map = id;
                                        self.history.clear();
                                    }
                                    Err(e) => self.status = e,
                                }
                            }
                        }
                    }
                }
            });
            if self.project_store.is_some() {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.new_map);
                    if ui.button("Add blank map").clicked() {
                        self.save();
                        if self.saved_scene.as_ref() == Some(&self.scene) {
                            let store = self.project_store.as_mut().unwrap();
                            if store.manifest.scenes.contains_key(&self.new_map) {
                                self.status = "Map ID exists".into();
                            } else {
                                match store.save_scene(&self.new_map, &Scene::default()) {
                                    Ok(()) => self.status = "Map added; select its tab".into(),
                                    Err(e) => self.status = e,
                                }
                            }
                        }
                    }
                });
            }
            ui.collapsing("Project · New / Open / Save", |ui| {
                ui.horizontal(|ui| {
                    ui.label("New folder / existing project folder");
                    ui.text_edit_singleline(&mut self.project_folder);
                });
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.project_name);
                });
                ui.checkbox(
                    &mut self.discard_confirmed,
                    "Discard unsaved edits when creating/opening",
                );
                ui.horizontal(|ui| {
                    if ui.button("New project").clicked() {
                        self.project_action(true);
                    }
                    if ui.button("Open project").clicked() {
                        self.project_action(false);
                    }
                    if ui.button("Save project").clicked() {
                        self.save();
                    }
                });
            });
        }
        if !playing && self.project_store.is_none() {
            ui.horizontal_wrapped(|ui| {
                ui.label("Scene file");
                ui.text_edit_singleline(&mut self.path);
                if ui.button("Open").clicked() {
                    self.load();
                }
                if ui.button("Save scene").clicked() {
                    self.save();
                }
                if ui.button("Undo paint").clicked() {
                    if let Some(s) = self.history.pop() {
                        self.scene = s;
                    }
                }
            });
        }
        if !playing {
            ui.horizontal_wrapped(|ui| {
                for (i, n) in [
                    "Snow",
                    "Path",
                    "Water",
                    "Tree",
                    "Building",
                    "NPC",
                    "Spawn",
                    "Portal",
                    "Objective",
                    "Remove portal",
                    "Collision",
                    "Light",
                    "Stamp",
                ]
                .iter()
                .enumerate()
                {
                    ui.selectable_value(&mut self.brush, i as u8, *n);
                }
                ui.add(egui::Slider::new(&mut self.elevation, 0..=4).text("Height"));
            });
            if self.brush == 7 {
                ui.horizontal(|ui| {
                    ui.label("Destination map");
                    ui.text_edit_singleline(&mut self.portal_target);
                    ui.add(egui::DragValue::new(&mut self.portal_spawn[0]).range(0..=15));
                    ui.add(egui::DragValue::new(&mut self.portal_spawn[1]).range(0..=15));
                });
            }
            ui.horizontal(|ui| {
                ui.label("NPC says");
                ui.text_edit_singleline(&mut self.scene.dialogue);
                ui.add(egui::Slider::new(&mut self.scene.ambient, 0.2..=1.0).text("Daylight"));
            });
        } else {
            ui.label("Arrow keys: walk • Enter: talk / next page • STOP: return to editor");
            if !ui.ctx().wants_keyboard_input() {
                for (key, dx, dy) in [
                    (egui::Key::ArrowLeft, -1, 0),
                    (egui::Key::ArrowRight, 1, 0),
                    (egui::Key::ArrowUp, 0, -1),
                    (egui::Key::ArrowDown, 0, 1),
                ] {
                    if ui.input(|i| i.key_pressed(key)) {
                        if let Some(runtime) = &mut self.runtime {
                            runtime.step(dx, dy);
                        }
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Some(runtime) = &mut self.runtime {
                        runtime.interact();
                    }
                }
                for (key, index) in [
                    (egui::Key::Num1, 0),
                    (egui::Key::Num2, 1),
                    (egui::Key::Num3, 2),
                    (egui::Key::Num4, 3),
                    (egui::Key::Num5, 4),
                    (egui::Key::Num6, 5),
                    (egui::Key::Num7, 6),
                    (egui::Key::Num8, 7),
                    (egui::Key::Num9, 8),
                ] {
                    if ui.input(|i| i.key_pressed(key)) {
                        if let Some(runtime) = &mut self.runtime {
                            runtime.choose(index);
                        }
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::Num1)) {
                    self.dialogue_choice = 0;
                }
                if ui.input(|i| i.key_pressed(egui::Key::Num2)) {
                    self.dialogue_choice = 1;
                }
            }
            if let Some(runtime) = &mut self.runtime {
                self.player = runtime.state.position;
                self.camera.smooth_follow(
                    [self.player[0] as f32, self.player[1] as f32],
                    [16, 16],
                    10.0,
                    1.0 / 60.0,
                );
                self.particles.update(&self.weather, 1.0 / 60.0);
                self.talking = runtime.page.is_some();
                ui.label(format!(
                    "Map: {}   Player: {:?}   Objective: {}",
                    runtime.state.map,
                    runtime.state.position,
                    if runtime.state.completed {
                        "complete"
                    } else {
                        "pending"
                    }
                ));
                ui.label(format!(
                    "Camera: {:.1}, {:.1}",
                    self.camera.center[0], self.camera.center[1]
                ));
                if let Some(store) = &self.project_store {
                    ui.horizontal(|ui| {
                        let path = store.root.join("saves/game.json");
                        if ui.button("Save game").clicked() {
                            self.status = serde_json::to_vec_pretty(&runtime.state)
                                .map_err(|e| e.to_string())
                                .and_then(|b| {
                                    let tmp = path.with_extension("pending");
                                    std::fs::write(&tmp, b)
                                        .and_then(|_| std::fs::rename(tmp, &path))
                                        .map_err(|e| e.to_string())
                                })
                                .map(|_| "Game saved".into())
                                .unwrap_or_else(|e| e);
                        }
                        if ui.button("Load game").clicked() {
                            self.status = std::fs::read(&path)
                                .map_err(|e| e.to_string())
                                .and_then(|b| serde_json::from_slice(&b).map_err(|e| e.to_string()))
                                .and_then(|s| runtime.restore(s))
                                .map(|_| "Game restored".into())
                                .unwrap_or_else(|e| e);
                        }
                    });
                }
            }
        }
        ui.label(&self.status);
        let (rect, response) = ui.allocate_exact_size(
            ui.available_size().max(Vec2::splat(10.0)),
            Sense::click_and_drag(),
        );
        let p = ui.painter_at(rect);
        p.rect_filled(rect, 0.0, Color32::from_rgb(27, 24, 35));
        let unit = (rect.width() / 18.0).min(rect.height() / 12.0).max(1.0);
        let origin = egui::pos2(rect.center().x - unit * 8.0, rect.top() + unit * 1.8);
        if !playing && (response.clicked() || response.dragged()) {
            if let Some(pos) = response.interact_pointer_pos() {
                let x = ((pos.x - origin.x) / unit).floor() as i32;
                let y = ((pos.y - origin.y) / (unit * 0.55)).floor() as i32;
                if (0..16).contains(&x) && (0..16).contains(&y) {
                    if response.drag_started() || response.clicked() {
                        if self.history.len() == 32 {
                            self.history.remove(0);
                        }
                        self.history.push(self.scene.clone());
                    }
                    let xy = [x as usize, y as usize];
                    let idx = xy[1] * 16 + xy[0];
                    match self.brush {
                        5 => {
                            self.scene.npc = xy;
                            if self.scene.npcs.is_empty() {
                                self.scene.npcs.push(xy);
                            } else {
                                self.scene.npcs[0] = xy;
                            }
                        }
                        6 => self.scene.spawn = xy,
                        7 => {
                            self.scene.portals.retain(|p| p.at != xy);
                            self.scene.portals.push(crate::runtime::Portal {
                                at: xy,
                                map: self.portal_target.clone(),
                                spawn: self.portal_spawn,
                            });
                        }
                        8 => self.scene.objective = Some(xy),
                        9 => self.scene.portals.retain(|p| p.at != xy),
                        10 => self.scene.collision[idx] = !self.scene.collision[idx],
                        11 => self.scene.light = xy,
                        12 => {
                            let stamp = crate::tilemap::TileStamp {
                                width: 2,
                                height: 2,
                                tiles: vec![1, 1, 1, 1],
                                collision: vec![false; 4],
                            };
                            if let Err(e) = stamp.apply(&mut self.scene, xy) {
                                self.status = e;
                            }
                        }
                        _ => {
                            self.scene.tiles[idx] = self.brush;
                            self.scene.heights[idx] = self.elevation;
                        }
                    }
                    self.status = "Unsaved scene changes".into();
                }
            }
        }
        let scene = if playing {
            self.runtime
                .as_ref()
                .map(|r| r.scene())
                .unwrap_or(&self.scene)
        } else {
            &self.scene
        };
        for y in 0..16 {
            for x in 0..16 {
                let i = y * 16 + x;
                let tile = scene.tiles[i];
                let h = scene.heights[i] as f32 * unit * 0.18;
                let pos = origin + egui::vec2(x as f32 * unit, y as f32 * unit * 0.55 - h);
                let top = Rect::from_min_size(pos, egui::vec2(unit, unit * 0.55));
                let rgb = match tile {
                    0 => [219, 234, 239],
                    1 => [158, 164, 174],
                    2 => [74, 126, 170],
                    _ => [203, 221, 226],
                };
                let light_origin = if playing { self.player } else { scene.light };
                let distance = ((x as f32 - light_origin[0] as f32).powi(2)
                    + (y as f32 - light_origin[1] as f32).powi(2))
                .sqrt();
                let falloff = (1.0 - distance / scene.light_radius).clamp(0.0, 1.0);
                let illumination =
                    (scene.ambient + falloff * (1.0 - scene.ambient)).clamp(0.2, 1.0);
                let c = Color32::from_rgb(
                    (rgb[0] as f32 * illumination) as u8,
                    (rgb[1] as f32 * illumination) as u8,
                    (rgb[2] as f32 * illumination) as u8,
                );
                p.rect_filled(
                    top.translate(egui::vec2(0.0, h)),
                    0.0,
                    Color32::from_rgb(93, 111, 137),
                );
                p.rect_filled(
                    Rect::from_min_size(top.left_bottom(), egui::vec2(unit, h)),
                    0.0,
                    Color32::from_rgb(131, 155, 180),
                );
                p.rect_filled(top, 0.0, c);
                if scene.portals.iter().any(|portal| portal.at == [x, y]) {
                    p.rect_stroke(top.shrink(2.0), 0.0, (2.0, Color32::from_rgb(210, 80, 190)));
                }
                if scene.objective == Some([x, y]) {
                    p.circle_filled(top.center(), unit * 0.14, Color32::YELLOW);
                }
                if scene.light == [x, y] {
                    p.circle_stroke(top.center(), unit * 0.22, (2.0, Color32::YELLOW));
                }
                if !playing {
                    p.rect_stroke(top, 0.0, (0.5, Color32::from_black_alpha(35)));
                    if scene.collision[i] {
                        p.rect_filled(
                            top.shrink(unit * 0.2),
                            0.0,
                            Color32::from_rgba_unmultiplied(210, 70, 120, 90),
                        );
                    }
                }
                if tile >= 3 {
                    p.rect_filled(
                        top.translate(egui::vec2(unit * 0.2, unit * 0.12)),
                        2.0,
                        Color32::from_black_alpha(65),
                    );
                    let object = Rect::from_min_size(
                        pos - egui::vec2(0.0, unit * 0.7),
                        egui::vec2(unit, unit),
                    );
                    p.rect_filled(
                        object,
                        0.0,
                        if tile == 3 {
                            Color32::from_rgb(44, 94, 76)
                        } else {
                            Color32::from_rgb(151, 132, 119)
                        },
                    );
                    p.rect_filled(
                        Rect::from_min_size(object.min, egui::vec2(unit, unit * 0.25)),
                        0.0,
                        Color32::from_rgb(237, 244, 248),
                    );
                }
                let mut actors = if scene.npcs.is_empty() {
                    vec![scene.npc]
                } else {
                    scene.npcs.clone()
                }
                .into_iter()
                .map(|xy| (xy, Color32::from_rgb(93, 161, 235)))
                .collect::<Vec<_>>();
                actors.push((
                    if playing { self.player } else { scene.spawn },
                    Color32::from_rgb(224, 67, 135),
                ));
                for (xy, col) in actors {
                    if xy == [x, y] {
                        let foot = top.center();
                        if col == Color32::from_rgb(224, 67, 135)
                            && scene
                                .sprite
                                .frames
                                .iter()
                                .any(|f| f.iter().any(|c| c[3] > 0))
                        {
                            let frame = if playing {
                                let tick = (ui.input(|i| i.time) * 1000.0
                                    / scene.sprite.frame_ms as f64)
                                    as usize;
                                self.runtime
                                    .as_ref()
                                    .and_then(|r| scene.character.frame(r.direction, true, tick))
                                    .unwrap_or(tick)
                            } else {
                                0
                            };
                            scene.sprite.draw(&p, foot, unit / 16.0, frame);
                            if playing {
                                ui.ctx()
                                    .request_repaint_after(std::time::Duration::from_millis(
                                        scene.sprite.frame_ms as u64,
                                    ));
                            }
                            continue;
                        }
                        p.rect_filled(
                            Rect::from_center_size(foot, egui::vec2(unit * 0.6, unit * 0.18)),
                            3.0,
                            Color32::from_black_alpha(90),
                        );
                        p.rect_filled(
                            Rect::from_min_size(
                                foot - egui::vec2(unit * 0.2, unit * 0.65),
                                egui::vec2(unit * 0.4, unit * 0.6),
                            ),
                            0.0,
                            col,
                        );
                        p.rect_filled(
                            Rect::from_min_size(
                                foot - egui::vec2(unit * 0.18, unit * 0.9),
                                egui::vec2(unit * 0.36, unit * 0.3),
                            ),
                            0.0,
                            Color32::from_rgb(241, 209, 174),
                        );
                    }
                }
            }
        }
        if playing {
            for particle in &self.particles.particles {
                let pos = origin
                    + egui::vec2(
                        particle.position[0] * unit,
                        particle.position[1] * unit * 0.55,
                    );
                if rect.contains(pos) {
                    p.circle_filled(
                        pos,
                        1.5,
                        Color32::from_rgba_unmultiplied(
                            self.weather.color[0],
                            self.weather.color[1],
                            self.weather.color[2],
                            self.weather.color[3],
                        ),
                    );
                }
            }
        }
        if self.talking && playing {
            let box_rect = Rect::from_min_size(
                rect.min + egui::vec2(12.0, 12.0),
                egui::vec2(rect.width() - 24.0, 85.0),
            );
            p.rect_filled(box_rect, 12.0, Color32::WHITE);
            p.rect_stroke(box_rect, 12.0, (2.0, Color32::DARK_GRAY));
            let text = p.layout(
                self.runtime
                    .as_ref()
                    .and_then(|r| r.line())
                    .unwrap_or("")
                    .to_string(),
                egui::FontId::proportional(18.0),
                Color32::BLACK,
                box_rect.width() - 32.0,
            );
            p.galley(box_rect.min + egui::vec2(16.0, 12.0), text, Color32::BLACK);
            if let Some(runtime) = self.runtime.as_ref() {
                if let Some(node) = runtime
                    .dialogue_node
                    .and_then(|i| scene.dialogue_nodes.get(i))
                {
                    for (i, choice) in node.choices.iter().enumerate() {
                        p.text(
                            box_rect.left_top() + egui::vec2(18.0, 52.0 + i as f32 * 18.0),
                            egui::Align2::LEFT_TOP,
                            format!("{}. {}", i + 1, choice.label),
                            egui::FontId::proportional(14.0),
                            Color32::DARK_GRAY,
                        );
                    }
                }
            }
        }
    }
}

pub struct Player(pub World);
impl eframe::App for Player {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.0.show(ui, true));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_scene_rejected() {
        let mut s = Scene::default();
        s.tiles.clear();
        assert!(s.validate().is_err());
    }
    #[test]
    fn roundtrip_and_collision() {
        let mut s = Scene::default();
        s.tiles[0] = 4;
        let restored: Scene = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert!(restored.validate().is_ok());
        assert!(!restored.walkable(0, 0));
        assert!(!restored.walkable(16, 0));
        assert!(restored.walkable(1, 0));
    }
}
