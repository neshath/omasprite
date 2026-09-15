use crate::{
    model::{MapStyle, Project, Tool, Workspace},
    theme, ui,
};
use eframe::egui::{self, Align, Color32, Layout, Sense, Vec2};

pub struct StudioApp {
    pub project: Project,
    pub workspace: Workspace,
    pub tool: Tool,
    pub frame: usize,
    pub onion_skin: bool,
    pub zoom: f32,
    pub selected_layer: usize,
    pub playing: bool,
    pub map_style: MapStyle,
    pub dialogue_open: bool,
    pub dialogue_speaker: String,
    pub dialogue_text: String,
    pub lighting_enabled: bool,
    pub shadows_enabled: bool,
    pub ambient: f32,
    pub light_radius: f32,
    pub particle_amount: f32,
    pub character_name: String,
}

impl StudioApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        Self {
            project: Project::default(),
            workspace: Workspace::Sprite,
            tool: Tool::Pencil,
            frame: 23,
            onion_skin: true,
            zoom: 4.0,
            selected_layer: 0,
            playing: false,
            map_style: MapStyle::Overworld,
            dialogue_open: true,
            dialogue_speaker: "Professor Rowan".into(),
            dialogue_text: "A new adventure starts with one small idea. What will you make?".into(),
            lighting_enabled: true,
            shadows_enabled: true,
            ambient: 0.35,
            light_radius: 0.65,
            particle_amount: 0.25,
            character_name: "Hero / winter outfit".into(),
        }
    }

    fn topbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            ui.label(egui::RichText::new("▣").size(26.0).color(theme::LIME));
            ui.label(egui::RichText::new("OMASPRITE").size(18.0).strong());
            ui.separator();
            ui.label(egui::RichText::new(self.workspace.label()).color(theme::MUTED));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("□  PLAY").clicked() {
                    self.playing = !self.playing;
                }
                if ui.button("▱  DIALOGUE").clicked() {
                    self.dialogue_open = !self.dialogue_open;
                }
                let _ = ui.button("⇩  EXPORT");
                let _ = ui.button("⌘  SAVE");
            });
        });
    }

    fn workspace_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (w, label) in [
                (Workspace::Sprite, "SPRITE"),
                (Workspace::World, "WORLD"),
                (Workspace::Logic, "LOGIC"),
                (Workspace::Character, "CHARACTER"),
                (Workspace::Effects, "FX"),
            ] {
                let selected = self.workspace == w;
                let fill = if selected {
                    theme::HOT
                } else {
                    theme::PANEL_DEEP
                };
                if ui
                    .add(egui::Button::new(egui::RichText::new(label).strong()).fill(fill))
                    .clicked()
                {
                    self.workspace = w;
                }
            }
            ui.separator();
            ui.label(egui::RichText::new("GAME EDITOR BETA").color(theme::LIME));
        });
    }

    fn canvas(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(available.x, available.y.max(410.0)),
            Sense::click_and_drag(),
        );
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 4.0, Color32::from_rgb(17, 14, 24));
        let scene = rect.shrink(20.0);
        if self.workspace == Workspace::World {
            self.map_canvas(&painter, scene);
        } else if self.workspace == Workspace::Character {
            self.character_canvas(&painter, scene);
        } else if self.workspace == Workspace::Effects {
            self.effects_canvas(&painter, scene);
        } else {
            self.sprite_canvas(&painter, scene);
        }
        if self.dialogue_open {
            self.dialogue_box(&painter, scene);
        }
        if response.hovered() {
            painter.circle_stroke(
                response.hover_pos().unwrap_or(scene.center()),
                8.0,
                (1.0, theme::LIME),
            );
        }
    }

    fn sprite_canvas(&self, painter: &egui::Painter, scene: egui::Rect) {
        painter.rect_filled(scene, 2.0, Color32::from_rgb(53, 27, 72));
        for y in 0..9 {
            let yy = scene.top() + y as f32 * scene.height() / 9.0;
            painter.line_segment(
                [egui::pos2(scene.left(), yy), egui::pos2(scene.right(), yy)],
                (1.0, Color32::from_rgb(91, 37, 112)),
            );
        }
        for x in 0..15 {
            let xx = scene.left() + x as f32 * scene.width() / 15.0;
            painter.line_segment(
                [egui::pos2(xx, scene.top()), egui::pos2(xx, scene.bottom())],
                (1.0, Color32::from_rgb(91, 37, 112)),
            );
        }
        let ground = egui::Rect::from_min_max(
            egui::pos2(scene.left(), scene.bottom() - scene.height() * 0.23),
            egui::pos2(scene.right(), scene.bottom()),
        );
        painter.rect_filled(ground, 0.0, Color32::from_rgb(126, 49, 45));
        painter.rect_filled(
            egui::Rect::from_min_max(
                ground.left_top(),
                egui::pos2(ground.right(), ground.top() + 18.0),
            ),
            0.0,
            Color32::from_rgb(49, 165, 72),
        );
        for i in 0..11 {
            let x = ground.left() + 20.0 + i as f32 * ground.width() / 12.0;
            painter.rect_filled(
                egui::Rect::from_min_size(egui::pos2(x, ground.top() + 4.0), egui::vec2(9.0, 22.0)),
                0.0,
                Color32::from_rgb(120, 218, 60),
            );
        }
        let hill = vec![
            egui::pos2(scene.left(), ground.top()),
            egui::pos2(
                scene.left() + scene.width() * 0.22,
                scene.top() + scene.height() * 0.55,
            ),
            egui::pos2(scene.left() + scene.width() * 0.42, ground.top()),
            egui::pos2(
                scene.left() + scene.width() * 0.62,
                scene.top() + scene.height() * 0.42,
            ),
            egui::pos2(scene.right(), ground.top()),
            egui::pos2(scene.right(), ground.top() + 20.0),
            egui::pos2(scene.left(), ground.top() + 20.0),
        ];
        painter.add(egui::Shape::convex_polygon(
            hill,
            Color32::from_rgb(18, 18, 28),
            egui::Stroke::NONE,
        ));
        let cx = scene.center().x;
        let cy = ground.top() - 54.0;
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(cx, cy), egui::vec2(26.0, 44.0)),
            3.0,
            Color32::WHITE,
        );
        painter.circle_filled(egui::pos2(cx, cy - 28.0), 15.0, Color32::WHITE);
        painter.circle_filled(egui::pos2(cx + 5.0, cy - 30.0), 2.5, Color32::BLACK);
        painter.line_segment(
            [
                egui::pos2(cx + 12.0, cy - 5.0),
                egui::pos2(cx + 36.0, cy - 24.0),
            ],
            (5.0, Color32::WHITE),
        );
        painter.circle_filled(egui::pos2(cx + 64.0, cy + 2.0), 18.0, theme::HOT);
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(cx + 64.0, cy + 27.0), egui::vec2(27.0, 25.0)),
            4.0,
            theme::HOT,
        );
        if self.onion_skin {
            painter.circle_filled(
                egui::pos2(cx - 61.0, cy - 20.0),
                13.0,
                Color32::from_rgba_unmultiplied(239, 68, 153, 70),
            );
        }
        painter.text(
            egui::pos2(scene.left() + 16.0, scene.top() + 14.0),
            egui::Align2::LEFT_TOP,
            "CUTE_ADVENTURE / FOREST_EDGE",
            egui::FontId::monospace(12.0),
            theme::MUTED,
        );
    }

    fn map_canvas(&self, painter: &egui::Painter, scene: egui::Rect) {
        painter.rect_filled(scene, 2.0, Color32::from_rgb(126, 170, 153));
        let tile_w = scene.width() / 16.0;
        let tile_h = scene.height() / 10.0;
        for row in 0..10 {
            for col in 0..16 {
                let rect = egui::Rect::from_min_size(
                    egui::pos2(
                        scene.left() + col as f32 * tile_w,
                        scene.top() + row as f32 * tile_h,
                    ),
                    egui::vec2(tile_w + 1.0, tile_h + 1.0),
                );
                let color = if row == 4 || row == 5 {
                    Color32::from_rgb(202, 206, 190)
                } else if (row + col) % 5 == 0 {
                    Color32::from_rgb(94, 151, 110)
                } else {
                    Color32::from_rgb(116, 169, 126)
                };
                painter.rect_filled(rect, 0.0, color);
            }
        }
        for &(x, y) in &[(2, 2), (12, 1), (14, 7), (4, 8)] {
            let px = scene.left() + x as f32 * tile_w + tile_w / 2.0;
            let py = scene.top() + y as f32 * tile_h + tile_h / 2.0;
            painter.circle_filled(
                egui::pos2(px, py),
                tile_w * 0.32,
                Color32::from_rgb(58, 104, 59),
            );
            painter.circle_filled(
                egui::pos2(px, py - tile_h * 0.18),
                tile_w * 0.28,
                Color32::from_rgb(70, 133, 68),
            );
        }
        let house = egui::Rect::from_min_size(
            egui::pos2(scene.left() + tile_w * 6.0, scene.top() + tile_h * 2.0),
            egui::vec2(tile_w * 4.0, tile_h * 3.0),
        );
        painter.rect_filled(house, 2.0, Color32::from_rgb(218, 173, 111));
        painter.rect_filled(
            egui::Rect::from_min_size(house.left_top(), egui::vec2(house.width(), tile_h * 0.7)),
            0.0,
            Color32::from_rgb(119, 72, 55),
        );
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(house.center().x - 8.0, house.bottom() - 25.0),
                egui::vec2(16.0, 25.0),
            ),
            0.0,
            Color32::from_rgb(76, 91, 108),
        );
        painter.circle_filled(
            egui::pos2(scene.center().x, scene.center().y + tile_h * 2.0),
            10.0,
            theme::HOT,
        );
        painter.text(
            egui::pos2(scene.left() + 14.0, scene.top() + 12.0),
            egui::Align2::LEFT_TOP,
            format!("MAP / {} / 16×10 CHUNK", self.map_style.label()),
            egui::FontId::monospace(12.0),
            Color32::from_rgb(34, 59, 47),
        );
    }

    fn dialogue_box(&self, painter: &egui::Painter, scene: egui::Rect) {
        let box_rect = egui::Rect::from_min_max(
            egui::pos2(scene.left() + 24.0, scene.bottom() - 118.0),
            egui::pos2(scene.right() - 24.0, scene.bottom() - 24.0),
        );
        painter.rect_filled(
            box_rect.translate(egui::vec2(3.0, 4.0)),
            10.0,
            Color32::from_black_alpha(130),
        );
        painter.rect_filled(box_rect, 10.0, Color32::from_rgb(243, 239, 226));
        painter.rect_stroke(box_rect, 10.0, (2.0, Color32::from_rgb(39, 31, 43)));
        painter.text(
            egui::pos2(box_rect.left() + 16.0, box_rect.top() + 12.0),
            egui::Align2::LEFT_TOP,
            &self.dialogue_speaker,
            egui::FontId::monospace(14.0),
            Color32::from_rgb(163, 36, 98),
        );
        painter.text(
            egui::pos2(box_rect.left() + 16.0, box_rect.top() + 36.0),
            egui::Align2::LEFT_TOP,
            &self.dialogue_text,
            egui::FontId::proportional(16.0),
            Color32::from_rgb(31, 25, 31),
        );
        painter.text(
            egui::pos2(box_rect.right() - 18.0, box_rect.bottom() - 15.0),
            egui::Align2::RIGHT_BOTTOM,
            "▼",
            egui::FontId::monospace(12.0),
            Color32::from_rgb(163, 36, 98),
        );
    }

    fn character_canvas(&self, painter: &egui::Painter, scene: egui::Rect) {
        painter.rect_filled(scene, 2.0, Color32::from_rgb(38, 29, 54));
        let grid = scene.shrink(24.0);
        for i in 0..16 {
            let x = grid.left() + i as f32 * grid.width() / 16.0;
            painter.line_segment(
                [egui::pos2(x, grid.top()), egui::pos2(x, grid.bottom())],
                (1.0, Color32::from_rgb(66, 52, 87)),
            );
        }
        for i in 0..12 {
            let y = grid.top() + i as f32 * grid.height() / 12.0;
            painter.line_segment(
                [egui::pos2(grid.left(), y), egui::pos2(grid.right(), y)],
                (1.0, Color32::from_rgb(66, 52, 87)),
            );
        }
        let cx = grid.center().x;
        let cy = grid.center().y + 40.0;
        painter.circle_filled(
            egui::pos2(cx, cy - 100.0),
            32.0,
            Color32::from_rgb(198, 221, 245),
        );
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(cx, cy - 38.0), egui::vec2(72.0, 105.0)),
            12.0,
            Color32::from_rgb(65, 129, 184),
        );
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(cx - 20.0, cy + 40.0), egui::vec2(18.0, 70.0)),
            5.0,
            Color32::from_rgb(44, 49, 69),
        );
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(cx + 20.0, cy + 40.0), egui::vec2(18.0, 70.0)),
            5.0,
            Color32::from_rgb(44, 49, 69),
        );
        painter.text(
            egui::pos2(grid.left() + 14.0, grid.top() + 12.0),
            egui::Align2::LEFT_TOP,
            "CHARACTER / 8 DIRECTIONS / 4 WALK FRAMES",
            egui::FontId::monospace(12.0),
            theme::MUTED,
        );
        painter.text(
            egui::pos2(grid.right() - 14.0, grid.top() + 12.0),
            egui::Align2::RIGHT_TOP,
            "IDLE · WALK · RUN · TALK",
            egui::FontId::monospace(12.0),
            theme::LIME,
        );
    }

    fn effects_canvas(&self, painter: &egui::Painter, scene: egui::Rect) {
        painter.rect_filled(scene, 2.0, Color32::from_rgb(12, 18, 35));
        let center = scene.center();
        for ring in [0.18, 0.32, 0.48] {
            painter.circle_stroke(
                center,
                scene.width() * ring,
                (2.0, Color32::from_rgba_unmultiplied(236, 104, 190, 80)),
            );
        }
        painter.circle_filled(center, 14.0, Color32::from_rgb(255, 190, 83));
        if self.lighting_enabled {
            painter.circle_filled(
                center,
                scene.width() * self.light_radius * 0.30,
                Color32::from_rgba_unmultiplied(255, 173, 84, 34),
            );
        }
        for i in 0..18 {
            let x = scene.left() + 20.0 + (i as f32 * 73.0) % scene.width();
            let y = scene.top() + 30.0 + ((i * 47) as f32 % scene.height());
            painter.circle_filled(egui::pos2(x, y), 2.0 + (i % 3) as f32, theme::SKY);
        }
        painter.text(
            egui::pos2(scene.left() + 14.0, scene.top() + 12.0),
            egui::Align2::LEFT_TOP,
            "LIGHTING LAB / PIXEL LIGHTS / SHADOWS / PARTICLES",
            egui::FontId::monospace(12.0),
            theme::MUTED,
        );
    }

    fn bottom(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("FRAME {:02} / 60", self.frame));
            if ui.button("◀").clicked() {
                self.frame = self.frame.saturating_sub(1);
            }
            if ui.button(if self.playing { "Ⅱ" } else { "▶" }).clicked() {
                self.playing = !self.playing;
            }
            if ui.button("■").clicked() {
                self.playing = false;
            }
            if ui.button("▶").clicked() {
                self.frame = (self.frame + 1).min(60);
            }
            ui.separator();
            ui.checkbox(&mut self.dialogue_open, "DIALOGUE PREVIEW");
            ui.separator();
            ui.checkbox(&mut self.onion_skin, "ONION SKIN");
            ui.separator();
            ui.label("ZOOM");
            ui.add(
                egui::Slider::new(&mut self.zoom, 1.0..=8.0)
                    .suffix("×")
                    .show_value(true),
            );
        });
    }
}

impl eframe::App for StudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.playing {
            self.frame = (self.frame + 1) % 60;
            ctx.request_repaint_after(std::time::Duration::from_millis(90));
        }
        egui::TopBottomPanel::top("topbar")
            .frame(egui::Frame::default().fill(theme::BG).inner_margin(10.0))
            .show(ctx, |ui| self.topbar(ui));
        egui::TopBottomPanel::bottom("status")
            .frame(
                egui::Frame::default()
                    .fill(theme::PANEL_DEEP)
                    .inner_margin(8.0),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("▣");
                    ui.label("OMARCHY LINUX");
                    ui.separator();
                    ui.label(
                        egui::RichText::new("MAKE GAMES. PIXEL BY PIXEL.").color(theme::MUTED),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(format!(
                            "PROJECT: {}  ·  {}",
                            self.project.name.to_uppercase(),
                            if self.project.saved {
                                "SAVED"
                            } else {
                                "UNSAVED"
                            }
                        ));
                    });
                });
            });
        egui::TopBottomPanel::top("tabs")
            .frame(
                egui::Frame::default()
                    .fill(theme::PANEL_DEEP)
                    .inner_margin(8.0),
            )
            .show(ctx, |ui| self.workspace_tabs(ui));
        egui::SidePanel::left("tools")
            .resizable(false)
            .exact_width(210.0)
            .frame(egui::Frame::default().fill(theme::PANEL).inner_margin(12.0))
            .show(ctx, |ui| ui::tools_panel(ui, self));
        egui::SidePanel::right("inspector")
            .resizable(true)
            .default_width(300.0)
            .frame(egui::Frame::default().fill(theme::PANEL).inner_margin(12.0))
            .show(ctx, |ui| ui::inspector_panel(ui, self));
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(theme::BG).inner_margin(12.0))
            .show(ctx, |ui| {
                self.canvas(ui);
                ui.separator();
                self.bottom(ui);
            });
    }
}
