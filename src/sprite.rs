use egui::{Color32, Rect, Sense};
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub frames: Vec<Vec<[u8; 4]>>,
    pub frame_ms: u32,
}
impl Default for Sprite {
    fn default() -> Self {
        Self {
            frames: vec![vec![[0; 4]; 384]],
            frame_ms: 160,
        }
    }
}
impl Sprite {
    pub fn fill(&mut self, frame: usize, x: usize, y: usize, color: [u8; 4]) {
        if frame >= self.frames.len() || x >= 16 || y >= 24 {
            return;
        }
        let pixels = &mut self.frames[frame];
        let old = pixels[y * 16 + x];
        if old == color {
            return;
        }
        let mut pending = vec![(x, y)];
        while let Some((x, y)) = pending.pop() {
            let index = y * 16 + x;
            if pixels[index] != old {
                continue;
            }
            pixels[index] = color;
            if x > 0 {
                pending.push((x - 1, y));
            }
            if x < 15 {
                pending.push((x + 1, y));
            }
            if y > 0 {
                pending.push((x, y - 1));
            }
            if y < 23 {
                pending.push((x, y + 1));
            }
        }
    }
    pub fn validate(&self) -> bool {
        !self.frames.is_empty()
            && self.frames.len() <= 32
            && self.frames.iter().all(|f| f.len() == 384)
            && (40..=2000).contains(&self.frame_ms)
    }
    pub fn draw(&self, p: &egui::Painter, foot: egui::Pos2, scale: f32, frame: usize) {
        for (y, row) in self.frames[frame % self.frames.len()]
            .chunks(16)
            .enumerate()
        {
            for (x, c) in row.iter().enumerate() {
                if c[3] > 0 {
                    p.rect_filled(
                        Rect::from_min_size(
                            foot + egui::vec2((x as f32 - 8.0) * scale, (y as f32 - 24.0) * scale),
                            egui::vec2(scale, scale),
                        ),
                        0.0,
                        Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]),
                    );
                }
            }
        }
    }
}
pub struct Editor {
    frame: usize,
    color: [u8; 4],
    eraser: bool,
    fill: bool,
    onion: bool,
    preview: bool,
    history: Vec<Sprite>,
    redo: Vec<Sprite>,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            frame: 0,
            color: [225, 70, 134, 255],
            eraser: false,
            fill: false,
            onion: true,
            preview: false,
            history: vec![],
            redo: vec![],
        }
    }
}
impl Editor {
    pub fn show(&mut self, ui: &mut egui::Ui, s: &mut Sprite) {
        ui.label("PLAYER SPRITE · 16 × 24 · saved with the active scene");
        ui.horizontal(|ui| {
            ui.color_edit_button_srgba_unmultiplied(&mut self.color);
            ui.checkbox(&mut self.eraser, "Eraser");
            ui.checkbox(&mut self.fill, "Fill");
            ui.checkbox(&mut self.onion, "Onion skin");
            ui.checkbox(&mut self.preview, "Animate");
            if ui.button("Undo").clicked() {
                if let Some(old) = self.history.pop() {
                    self.redo.push(s.clone());
                    *s = old;
                }
            }
            if ui.button("Redo").clicked() {
                if let Some(next) = self.redo.pop() {
                    self.history.push(s.clone());
                    *s = next;
                }
            }
        });
        ui.horizontal_wrapped(|ui| {
            for (i, name) in ["Ink", "Snow", "Rose", "Blue", "Gold"].iter().enumerate() {
                if ui.button(*name).clicked() {
                    self.color = [
                        [32, 28, 44, 255],
                        [235, 243, 248, 255],
                        [225, 70, 134, 255],
                        [74, 138, 201, 255],
                        [232, 181, 78, 255],
                    ][i];
                }
            }
        });
        ui.horizontal_wrapped(|ui| {
            for i in 0..s.frames.len() {
                ui.selectable_value(&mut self.frame, i, format!("Frame {}", i + 1));
            }
            if s.frames.len() < 32 && ui.button("Duplicate frame").clicked() {
                self.history.push(s.clone());
                self.redo.clear();
                s.frames
                    .push(s.frames[self.frame.min(s.frames.len() - 1)].clone());
                self.frame = s.frames.len() - 1;
            }
            ui.add(egui::Slider::new(&mut s.frame_ms, 40..=2000).text("ms/frame"));
            if s.frames.len() > 1 && ui.button("Delete frame").clicked() {
                self.history.push(s.clone());
                self.redo.clear();
                s.frames.remove(self.frame.min(s.frames.len() - 1));
            }
        });
        self.frame = self.frame.min(s.frames.len() - 1);
        let scale = (ui.available_height() / 24.0)
            .min(ui.available_width() / 16.0)
            .floor()
            .clamp(1.0, 20.0);
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(16.0 * scale, 24.0 * scale),
            Sense::click_and_drag(),
        );
        if !self.preview && (response.clicked() || response.dragged()) {
            if let Some(pos) = response.interact_pointer_pos() {
                let x = ((pos.x - rect.left()) / scale) as usize;
                let y = ((pos.y - rect.top()) / scale) as usize;
                if x < 16 && y < 24 {
                    if response.clicked() || response.drag_started() {
                        if self.history.len() == 32 {
                            self.history.remove(0);
                        }
                        self.history.push(s.clone());
                        self.redo.clear();
                    }
                    let color = if self.eraser { [0; 4] } else { self.color };
                    if self.fill {
                        s.fill(self.frame, x, y, color);
                    } else {
                        s.frames[self.frame][y * 16 + x] = color;
                    }
                }
            }
        }
        let p = ui.painter_at(rect);
        for y in 0..24 {
            for x in 0..16 {
                p.rect_filled(
                    Rect::from_min_size(
                        rect.min + egui::vec2(x as f32 * scale, y as f32 * scale),
                        egui::vec2(scale, scale),
                    ),
                    0.0,
                    if (x + y) % 2 == 0 {
                        Color32::from_gray(50)
                    } else {
                        Color32::from_gray(65)
                    },
                );
            }
        }
        if self.onion && self.frame > 0 && !self.preview {
            let previous = Sprite {
                frames: vec![s.frames[self.frame - 1]
                    .iter()
                    .map(|c| [c[0], c[1], c[2], c[3] / 3])
                    .collect()],
                frame_ms: s.frame_ms,
            };
            previous.draw(&p, egui::pos2(rect.center().x, rect.bottom()), scale, 0);
        }
        let frame = if self.preview {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(s.frame_ms as u64));
            (ui.input(|i| i.time) * 1000.0 / s.frame_ms as f64) as usize
        } else {
            self.frame
        };
        s.draw(&p, egui::pos2(rect.center().x, rect.bottom()), scale, frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fill_respects_connected_boundary_and_roundtrips() {
        let mut s = Sprite::default();
        for y in 0..24 {
            s.frames[0][y * 16 + 8] = [1, 2, 3, 255];
        }
        s.fill(0, 0, 0, [7, 8, 9, 255]);
        assert_eq!(s.frames[0][23 * 16], [7, 8, 9, 255]);
        assert_eq!(s.frames[0][9], [0; 4]);
        assert_eq!(s.frames[0][8], [1, 2, 3, 255]);
        let restored: Sprite = serde_json::from_slice(&serde_json::to_vec(&s).unwrap()).unwrap();
        assert!(restored.validate());
        assert_eq!(restored.frames, s.frames);
    }
}
