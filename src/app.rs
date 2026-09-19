use crate::{
    model::{MapStyle, Project, Tool, Workspace},
    theme, ui,
};
use eframe::egui::{self, Align, Color32, Layout, Sense, Vec2};

pub struct StudioApp {
    reference_skin: bool,
    reference_texture: egui::TextureHandle,
    world: crate::world::World,
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
    pub logic_nodes: Vec<crate::advanced::LogicNode>,
    pub keyboard: crate::advanced::KeyboardBindings,
    pub accessibility: crate::advanced::AccessibilityAudit,
}

impl StudioApp {
    pub fn open_project(&mut self, path: String) {
        self.world = crate::world::World::from_file(path);
    }
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        let bytes = include_bytes!("../assets/omarchy-studio-reference.png");
        let source = image::load_from_memory(bytes)
            .expect("embedded reference skin")
            .to_rgba8();
        // Keep the supplied artwork intact while respecting common 2048px GPU limits.
        // Half resolution is the exact source aspect ratio: 2700x1568 -> 1350x784.
        let rgba =
            image::imageops::resize(&source, 1350, 784, image::imageops::FilterType::Lanczos3);
        let size = [rgba.width() as usize, rgba.height() as usize];
        let reference_texture = cc.egui_ctx.load_texture(
            "exact-omasprite-reference",
            egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()),
            egui::TextureOptions::NEAREST,
        );
        Self {
            reference_skin: false,
            reference_texture,
            world: crate::world::World::default(),
            project: Project::default(),
            workspace: Workspace::World,
            tool: Tool::Pencil,
            frame: 23,
            onion_skin: true,
            zoom: 4.0,
            selected_layer: 0,
            playing: false,
            map_style: MapStyle::Overworld,
            dialogue_open: true,
            dialogue_speaker: "Mira".into(),
            dialogue_text: "A new adventure starts with one small idea. What will you make?".into(),
            lighting_enabled: true,
            shadows_enabled: true,
            ambient: 0.35,
            light_radius: 0.65,
            particle_amount: 0.25,
            character_name: "Hero / winter outfit".into(),
            logic_nodes: vec![crate::advanced::LogicNode {
                id: "start".into(),
                kind: "event".into(),
                inputs: vec![],
                outputs: vec!["next".into()],
            }],
            keyboard: crate::advanced::KeyboardBindings {
                up: "ArrowUp".into(),
                down: "ArrowDown".into(),
                left: "ArrowLeft".into(),
                right: "ArrowRight".into(),
                interact: "Enter".into(),
                play: "Space".into(),
            },
            accessibility: crate::advanced::AccessibilityAudit {
                labels: vec![
                    "topbar".into(),
                    "workspace tabs".into(),
                    "canvas".into(),
                    "inspector".into(),
                    "transport".into(),
                ],
                keyboard_complete: false,
                contrast_checked: true,
            },
        }
    }

    fn topbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("OMASPRITE").size(18.0).strong());
            ui.label(
                egui::RichText::new("Game Editor Beta")
                    .small()
                    .color(theme::MUTED),
            );
            ui.separator();
            ui.label(egui::RichText::new(self.world.project_label()).color(theme::MUTED));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let play_label = if self.playing { "Stop" } else { "Play" };
                if ui
                    .add(egui::Button::new(play_label).fill(if self.playing {
                        theme::PANEL_RAISED
                    } else {
                        theme::HOT
                    }))
                    .clicked()
                {
                    self.playing = !self.playing;
                    if self.playing {
                        self.world.start();
                    }
                    self.workspace = Workspace::World;
                }
                if ui.button("Save").clicked() {
                    self.world.save();
                }
                if ui
                    .button("Reference")
                    .on_hover_text("Open the supplied reference artwork")
                    .clicked()
                {
                    self.reference_skin = true;
                }
                if ui.button("Dialogue").clicked() {
                    self.dialogue_open = !self.dialogue_open;
                }
            });
        });
    }

    fn workspace_tabs(&mut self, ui: &mut egui::Ui) {
        egui::Frame::default()
            .fill(theme::PANEL)
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(3.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (w, label) in [
                        (Workspace::Sprite, "Sprite"),
                        (Workspace::World, "World"),
                        (Workspace::Logic, "Logic"),
                        (Workspace::Character, "Character"),
                        (Workspace::Effects, "Effects"),
                    ] {
                        let selected = self.workspace == w;
                        let response = ui.add(egui::Button::new(label).fill(if selected {
                            theme::HOT
                        } else {
                            Color32::TRANSPARENT
                        }));
                        if response.clicked() {
                            self.workspace = w;
                        }
                    }
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("F1  Reference")
                            .small()
                            .color(theme::MUTED),
                    );
                });
            });
    }

    fn contextbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(self.workspace.label()).strong());
            ui.separator();
            ui.label(
                egui::RichText::new("Create, preview, refine")
                    .small()
                    .color(theme::MUTED),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("16 × 16 scene")
                        .small()
                        .color(theme::MUTED),
                );
            });
        });
    }

    fn canvas(&mut self, ui: &mut egui::Ui) {
        if self.playing {
            self.world.show(ui, true);
            return;
        }
        if self.workspace == Workspace::Sprite {
            self.world.sprite_ui(ui);
            return;
        }
        if self.workspace == Workspace::World {
            self.world.show(ui, self.playing);
            return;
        }
        self.authoring_controls(ui);
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

    fn authoring_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| match self.workspace {
            Workspace::Logic => {
                ui.label("LOGIC GRAPH");
                if ui.button("+ Event node").clicked() {
                    let id = format!("event-{}", self.logic_nodes.len());
                    self.logic_nodes.push(crate::advanced::LogicNode {
                        id,
                        kind: "event".into(),
                        inputs: vec![],
                        outputs: vec!["next".into()],
                    });
                }
                if ui.button("Export Lua").clicked() {
                    self.dialogue_text = crate::advanced::lua_export(&self.logic_nodes);
                }
                ui.label(format!(
                    "{} nodes · deterministic preview",
                    self.logic_nodes.len()
                ));
            }
            Workspace::Character => {
                ui.label("DIRECTIONS");
                for direction in ["UP", "DOWN", "LEFT", "RIGHT"] {
                    let _ = ui.selectable_label(true, direction);
                }
                ui.label("IDLE / WALK / TALK");
            }
            Workspace::Effects => {
                ui.label("EMITTER");
                ui.add(egui::Slider::new(&mut self.particle_amount, 0.0..=1.0).text("rate"));
                ui.checkbox(&mut self.shadows_enabled, "Palette shadows");
            }
            _ => {}
        });
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
        painter.circle_filled(egui::pos2(cx + 64.0, cy + 2.0), 18.0, theme::BRAND);
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(cx + 64.0, cy + 27.0), egui::vec2(27.0, 25.0)),
            4.0,
            theme::BRAND,
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
            ui.label(egui::RichText::new(format!("Frame {} / 60", self.frame)).color(theme::MUTED));
            if ui
                .button("Previous")
                .on_hover_text("Previous frame")
                .clicked()
            {
                self.frame = self.frame.saturating_sub(1);
            }
            if ui
                .button(if self.playing { "Pause" } else { "Play" })
                .clicked()
            {
                self.playing = !self.playing;
                if self.playing {
                    self.world.start();
                    self.workspace = Workspace::World;
                }
            }
            if ui.button("Stop").clicked() {
                self.playing = false;
            }
            if ui.button("Next").on_hover_text("Next frame").clicked() {
                self.frame = (self.frame + 1).min(60);
            }
            ui.separator();
            ui.checkbox(&mut self.dialogue_open, "Dialogue preview");
            ui.checkbox(&mut self.onion_skin, "Onion skin");
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
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.reference_skin = !self.reference_skin;
        }
        if self.reference_skin {
            egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    let rect = ui.max_rect();
                    let source_ratio = 2700.0 / 1568.0;
                    let mut image_size = rect.size();
                    if image_size.x / image_size.y > source_ratio {
                        image_size.x = image_size.y * source_ratio;
                    } else {
                        image_size.y = image_size.x / source_ratio;
                    }
                    let image_rect = egui::Rect::from_center_size(rect.center(), image_size);
                    ui.painter().rect_filled(rect, 0.0, Color32::BLACK);
                    ui.painter().image(
                        self.reference_texture.id(),
                        image_rect,
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    let hint = egui::Rect::from_min_size(
                        image_rect.left_top() + egui::vec2(8.0, 8.0),
                        egui::vec2(178.0, 24.0),
                    );
                    let response = ui.allocate_rect(hint, egui::Sense::click());
                    if response.clicked() {
                        self.reference_skin = false;
                    }
                    response.on_hover_text(
                        "Exact reference skin · click logo or press F1 for the functional editor",
                    );
                });
            return;
        }
        ctx.input(|input| {
            if input.key_pressed(egui::Key::Space) && !ctx.wants_keyboard_input() {
                self.playing = !self.playing;
                if self.playing {
                    self.world.start();
                }
            }
            if input.key_pressed(egui::Key::Escape) {
                self.playing = false;
            }
        });
        if self.playing {
            self.frame = (self.frame + 1) % 60;
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
        egui::TopBottomPanel::top("topbar")
            .frame(
                egui::Frame::default()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(10.0, 8.0)),
            )
            .show(ctx, |ui| self.topbar(ui));
        egui::TopBottomPanel::bottom("status")
            .frame(
                egui::Frame::default()
                    .fill(theme::PANEL_DEEP)
                    .inner_margin(egui::Margin::symmetric(10.0, 7.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&self.world.status).color(theme::MUTED));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("Space Play  ·  F1 Reference")
                                .small()
                                .color(theme::MUTED),
                        );
                    });
                });
            });
        egui::TopBottomPanel::top("tabs")
            .frame(
                egui::Frame::default()
                    .fill(theme::PANEL_DEEP)
                    .inner_margin(egui::Margin::symmetric(6.0, 5.0)),
            )
            .show(ctx, |ui| self.workspace_tabs(ui));
        egui::TopBottomPanel::top("contextbar")
            .frame(
                egui::Frame::default()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(16.0, 6.0)),
            )
            .show(ctx, |ui| self.contextbar(ui));
        egui::SidePanel::left("navigator")
            .resizable(true)
            .default_width(236.0)
            .min_width(210.0)
            .frame(
                egui::Frame::default()
                    .fill(theme::PANEL_DEEP)
                    .inner_margin(egui::Margin::symmetric(14.0, 12.0)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| ui::tools_panel(ui, self));
            });
        egui::SidePanel::right("inspector")
            .resizable(true)
            .default_width(312.0)
            .frame(
                egui::Frame::default()
                    .fill(theme::PANEL)
                    .inner_margin(egui::Margin::symmetric(16.0, 14.0)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui::inspector_panel(ui, self);
                    ui.collapsing("Keyboard and accessibility", |ui| {
                        ui.label(format!("Play: {} · Stop: Escape", self.keyboard.play));
                        ui.label(format!("Move: {}/{}/{}/{} · Talk: {}", self.keyboard.up, self.keyboard.down, self.keyboard.left, self.keyboard.right, self.keyboard.interact));
                        ui.label(format!("{} labelled regions · contrast {} · keyboard audit {}", self.accessibility.labels.len(), if self.accessibility.contrast_checked { "checked" } else { "pending" }, if self.accessibility.keyboard_complete { "complete" } else { "in progress" }));
                        ui.label("Tab moves between controls. Space activates Play. Number keys select dialogue choices.");
                    });
                });
            });
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0)),
            )
            .show(ctx, |ui| {
                egui::TopBottomPanel::bottom("transport")
                    .frame(
                        egui::Frame::default()
                            .fill(theme::PANEL_DEEP)
                            .inner_margin(egui::Margin::symmetric(8.0, 6.0)),
                    )
                    .show_inside(ui, |ui| self.bottom(ui));
                self.canvas(ui);
            });
    }
}
