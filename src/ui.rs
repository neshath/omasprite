use crate::{
    app::StudioApp,
    model::{MapStyle, Tool, Workspace, LEVELS},
    theme,
};
use eframe::egui::{self, Align, Color32, Layout};

fn section_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(10.0);
    ui.label(
        egui::RichText::new(title)
            .size(13.0)
            .strong()
            .color(theme::MUTED),
    );
}

pub fn tools_panel(ui: &mut egui::Ui, app: &mut StudioApp) {
    ui.label(egui::RichText::new("Navigator").size(20.0).strong());
    ui.label(egui::RichText::new("Your project, in one place.").color(theme::MUTED));
    section_heading(ui, "Project");
    ui.label(egui::RichText::new(&app.project.name).strong());
    ui.label(
        egui::RichText::new(&app.project.path)
            .small()
            .color(theme::MUTED),
    );
    ui.separator();
    section_heading(ui, "Library");
    for (workspace, label, detail) in [
        (Workspace::World, "Scenes", "Snow Village"),
        (Workspace::Sprite, "Sprites", "Hero and NPCs"),
        (Workspace::Character, "Characters", "Directional sets"),
        (Workspace::Logic, "Dialogue & logic", "Events and choices"),
        (
            Workspace::Effects,
            "Lighting & effects",
            "Weather and shadows",
        ),
    ] {
        if ui
            .selectable_label(
                app.workspace == workspace,
                egui::RichText::new(label).strong(),
            )
            .on_hover_text(detail)
            .clicked()
        {
            app.workspace = workspace;
        }
    }
    ui.separator();
    section_heading(ui, "Tools");
    for row in [
        [Tool::Pencil, Tool::Fill, Tool::Eraser],
        [Tool::Select, Tool::Stamp, Tool::Light],
        [Tool::Shadow, Tool::Path, Tool::Particle],
    ] {
        ui.horizontal(|ui| {
            for tool in row {
                let selected = app.tool == tool;
                if ui
                    .add_sized(
                        [82.0, 40.0],
                        egui::Button::new(egui::RichText::new(tool.label()).size(11.0)).fill(
                            if selected {
                                theme::HOT
                            } else {
                                theme::PANEL_RAISED
                            },
                        ),
                    )
                    .on_hover_text(tool.label())
                    .clicked()
                {
                    app.tool = tool;
                }
            }
        });
    }
    ui.separator();
    section_heading(ui, "Palette");
    let colors = [
        theme::BRAND,
        theme::VIOLET,
        theme::LIME,
        theme::SKY,
        Color32::WHITE,
        Color32::BLACK,
        Color32::from_rgb(126, 49, 45),
        Color32::from_rgb(43, 160, 92),
        Color32::from_rgb(246, 182, 76),
        Color32::from_rgb(92, 65, 148),
        Color32::from_rgb(255, 115, 186),
        Color32::from_rgb(90, 211, 191),
    ];
    ui.horizontal_wrapped(|ui| {
        for c in colors {
            ui.add(
                egui::Button::new("  ")
                    .fill(c)
                    .min_size(egui::vec2(29.0, 24.0)),
            );
        }
    });
    ui.separator();
    section_heading(ui, "Layers");
    for (i, layer) in ["BG", "COLLISION", "MAIN CHAR", "SIDE CHAR"]
        .iter()
        .enumerate()
    {
        let selected = app.selected_layer == i;
        if ui
            .selectable_label(
                selected,
                format!("{}  {}", if selected { "●" } else { "○" }, layer),
            )
            .clicked()
        {
            app.selected_layer = i;
        }
    }
    ui.separator();
    section_heading(ui, "Assets");
    ui.horizontal_wrapped(|ui| {
        for (label, color) in [
            ("HERO", theme::BRAND),
            ("TREE", theme::LIME),
            ("TILE", theme::SKY),
            ("FX", theme::VIOLET),
        ] {
            ui.add(
                egui::Button::new(egui::RichText::new(label).size(11.0))
                    .fill(color)
                    .min_size(egui::vec2(60.0, 38.0)),
            );
        }
    });
    ui.add_space(10.0);
    ui.collapsing("Creator progress", |ui| {
        ui.add(
            egui::ProgressBar::new(app.project.level as f32 / 20.0)
                .text(format!("Level {}", app.project.level)),
        );
        ui.label(
            egui::RichText::new("Next: lighting and shadows")
                .small()
                .color(theme::MUTED),
        );
    });
}

pub fn inspector_panel(ui: &mut egui::Ui, app: &mut StudioApp) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Inspector").size(20.0).strong());
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                egui::RichText::new("Properties")
                    .small()
                    .color(theme::MUTED),
            );
        });
    });
    ui.label(egui::RichText::new(app.workspace.label()).color(theme::LIME));
    ui.label(
        egui::RichText::new("Select an item in the canvas to edit its properties.")
            .small()
            .color(theme::MUTED),
    );
    ui.separator();
    section_heading(ui, "Project");
    ui.label(egui::RichText::new(&app.project.name).strong());
    ui.label(
        egui::RichText::new(&app.project.path)
            .small()
            .color(theme::MUTED),
    );
    ui.separator();
    if app.workspace == Workspace::World {
        section_heading(ui, "Map");
        for style in [MapStyle::Overworld, MapStyle::Town, MapStyle::Interior] {
            if ui
                .selectable_label(app.map_style == style, style.label())
                .clicked()
            {
                app.map_style = style;
            }
        }
        ui.label(
            egui::RichText::new("Layers: ground · collision · objects · lighting")
                .small()
                .color(theme::MUTED),
        );
        ui.separator();
    }
    if app.workspace == Workspace::Character {
        section_heading(ui, "Character");
        ui.text_edit_singleline(&mut app.character_name);
        for item in [
            "Direction set: 8-way",
            "Animation: idle / walk / talk",
            "Palette: winter daylight",
        ] {
            ui.label(egui::RichText::new(item).small().color(theme::MUTED));
        }
        ui.separator();
    }
    if app.workspace == Workspace::Effects {
        section_heading(ui, "Lighting and effects");
        ui.checkbox(&mut app.lighting_enabled, "Pixel lighting");
        ui.checkbox(&mut app.shadows_enabled, "Cast sprite shadows");
        ui.add(egui::Slider::new(&mut app.ambient, 0.0..=1.0).text("Ambient"));
        ui.add(egui::Slider::new(&mut app.light_radius, 0.1..=1.0).text("Light radius"));
        ui.add(egui::Slider::new(&mut app.particle_amount, 0.0..=1.0).text("Particle density"));
        ui.label(egui::RichText::new("Planned: palette-aware normal maps, weather presets, bloom, and GPU shader graphs.").small().color(theme::MUTED));
        ui.separator();
    }
    section_heading(ui, "Dialogue");
    ui.checkbox(&mut app.dialogue_open, "Preview in scene");
    ui.horizontal(|ui| {
        ui.label("Speaker");
        ui.text_edit_singleline(&mut app.dialogue_speaker);
    });
    ui.label("Line");
    ui.text_edit_multiline(&mut app.dialogue_text);
    ui.label(
        egui::RichText::new(
            "Supports portrait, choices, speaker style, and event hooks in the next beta slice.",
        )
        .small()
        .color(theme::MUTED),
    );
    ui.separator();
    ui.collapsing("Project assets", |ui| {
        for asset in [
            "hero_idle.png",
            "hero_walk.png",
            "forest_tiles.png",
            "pink_slime.png",
            "torch_glow.png",
        ] {
            let _ = ui.selectable_label(false, asset);
        }
    });
    ui.collapsing("Creator path", |ui| {
        for (level, title, detail) in LEVELS.iter() {
            let unlocked = app.project.level >= *level;
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(if unlocked { "●" } else { "○" }).color(if unlocked {
                        theme::LIME
                    } else {
                        theme::MUTED
                    }),
                );
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{}  {}", level, title))
                            .strong()
                            .color(if unlocked { theme::INK } else { theme::MUTED }),
                    );
                    ui.label(egui::RichText::new(*detail).small().color(theme::MUTED));
                });
            });
        }
    });
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new("Tip: press Space to preview your game.")
            .small()
            .color(theme::MUTED),
    );
}
