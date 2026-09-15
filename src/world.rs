//! Editable height-field scene and small native RPG runtime.
use egui::{self, Color32, Rect, Sense, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Scene {
    pub version: u32,
    pub tiles: Vec<u8>,
    pub heights: Vec<u8>,
    pub npc: [usize; 2],
    pub spawn: [usize; 2],
    pub dialogue: String,
    pub ambient: f32,
}
impl Default for Scene {
    fn default() -> Self {
        Self {
            version: 1,
            tiles: vec![0; 256],
            heights: vec![0; 256],
            npc: [8, 7],
            spawn: [8, 9],
            dialogue: "Welcome! Build your village and tell its story.".into(),
            ambient: 0.85,
        }
    }
}
impl Scene {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1
            || self.tiles.len() != 256
            || self.heights.len() != 256
            || self.tiles.iter().any(|v| *v > 4)
            || self.heights.iter().any(|v| *v > 4)
            || self.npc.iter().chain(self.spawn.iter()).any(|v| *v >= 16)
            || !self.ambient.is_finite()
            || !(0.2..=1.0).contains(&self.ambient)
        {
            return Err("Invalid scene data".into());
        }
        Ok(())
    }
    pub fn walkable(&self, x: usize, y: usize) -> bool {
        x < 16 && y < 16 && self.tiles[y * 16 + x] < 2
    }
}
pub struct World {
    pub scene: Scene,
    pub path: String,
    pub status: String,
    brush: u8,
    elevation: u8,
    player: [usize; 2],
    talking: bool,
    history: Vec<Scene>,
}
impl Default for World {
    fn default() -> Self {
        Self {
            scene: Scene::default(),
            path: "scene.omasprite.json".into(),
            status: "New unsaved scene".into(),
            brush: 0,
            elevation: 0,
            player: [8, 9],
            talking: false,
            history: vec![],
        }
    }
}
impl World {
    pub fn from_file(path: String) -> Self {
        let mut world = Self {
            path,
            ..Default::default()
        };
        world.load();
        world
    }
    pub fn start(&mut self) {
        self.player = self.scene.spawn;
        self.talking = false;
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
                self.history.clear();
                self.start();
                self.status = "Scene loaded".into();
            }
            Err(e) => self.status = e,
        }
    }
    pub fn save(&mut self) {
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
            Ok(()) => "Scene saved. Share this JSON with another Omasprite installation.".into(),
            Err(e) => e,
        };
    }
    pub fn show(&mut self, ui: &mut egui::Ui, playing: bool) {
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
        if !playing {
            ui.horizontal_wrapped(|ui| {
                for (i, n) in ["Snow", "Path", "Water", "Tree", "Building", "NPC", "Spawn"]
                    .iter()
                    .enumerate()
                {
                    ui.selectable_value(&mut self.brush, i as u8, *n);
                }
                ui.add(egui::Slider::new(&mut self.elevation, 0..=4).text("Height"));
            });
            ui.horizontal(|ui| {
                ui.label("NPC says");
                ui.text_edit_singleline(&mut self.scene.dialogue);
                ui.add(egui::Slider::new(&mut self.scene.ambient, 0.2..=1.0).text("Daylight"));
            });
        } else {
            ui.label("Arrow keys: walk • Enter: talk to nearby NPC / close dialogue • PLAY: return to editor");
            if !ui.ctx().wants_keyboard_input() {
                for (key, dx, dy) in [
                    (egui::Key::ArrowLeft, -1, 0),
                    (egui::Key::ArrowRight, 1, 0),
                    (egui::Key::ArrowUp, 0, -1),
                    (egui::Key::ArrowDown, 0, 1),
                ] {
                    if !self.talking && ui.input(|i| i.key_pressed(key)) {
                        let x = self.player[0] as i32 + dx;
                        let y = self.player[1] as i32 + dy;
                        if x >= 0
                            && y >= 0
                            && self.scene.walkable(x as usize, y as usize)
                            && [x as usize, y as usize] != self.scene.npc
                            && self.scene.heights[y as usize * 16 + x as usize]
                                .abs_diff(self.scene.heights[self.player[1] * 16 + self.player[0]])
                                <= 1
                        {
                            self.player = [x as usize, y as usize];
                        }
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::Enter))
                    && self.player[0].abs_diff(self.scene.npc[0])
                        + self.player[1].abs_diff(self.scene.npc[1])
                        <= 1
                {
                    self.talking = !self.talking;
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
                        5 => self.scene.npc = xy,
                        6 => self.scene.spawn = xy,
                        _ => {
                            self.scene.tiles[idx] = self.brush;
                            self.scene.heights[idx] = self.elevation;
                        }
                    }
                    self.status = "Unsaved scene changes".into();
                }
            }
        }
        for y in 0..16 {
            for x in 0..16 {
                let i = y * 16 + x;
                let tile = self.scene.tiles[i];
                let h = self.scene.heights[i] as f32 * unit * 0.18;
                let pos = origin + egui::vec2(x as f32 * unit, y as f32 * unit * 0.55 - h);
                let top = Rect::from_min_size(pos, egui::vec2(unit, unit * 0.55));
                let rgb = match tile {
                    0 => [219, 234, 239],
                    1 => [158, 164, 174],
                    2 => [74, 126, 170],
                    _ => [203, 221, 226],
                };
                let c = Color32::from_rgb(
                    (rgb[0] as f32 * self.scene.ambient) as u8,
                    (rgb[1] as f32 * self.scene.ambient) as u8,
                    (rgb[2] as f32 * self.scene.ambient) as u8,
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
                if !playing {
                    p.rect_stroke(top, 0.0, (0.5, Color32::from_black_alpha(35)));
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
                for (xy, col) in [
                    (self.scene.npc, Color32::from_rgb(93, 161, 235)),
                    (
                        if playing {
                            self.player
                        } else {
                            self.scene.spawn
                        },
                        Color32::from_rgb(224, 67, 135),
                    ),
                ] {
                    if xy == [x, y] {
                        let foot = top.center();
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
        if self.talking && playing {
            let box_rect = Rect::from_min_size(
                rect.min + egui::vec2(12.0, 12.0),
                egui::vec2(rect.width() - 24.0, 85.0),
            );
            p.rect_filled(box_rect, 12.0, Color32::WHITE);
            p.rect_stroke(box_rect, 12.0, (2.0, Color32::DARK_GRAY));
            let text = p.layout(
                self.scene.dialogue.clone(),
                egui::FontId::proportional(18.0),
                Color32::BLACK,
                box_rect.width() - 32.0,
            );
            p.galley(box_rect.min + egui::vec2(16.0, 12.0), text, Color32::BLACK);
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
