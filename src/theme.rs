use egui::{Color32, Context, FontFamily, FontId, Rounding, Style, TextStyle, Visuals};

pub const INK: Color32 = Color32::from_rgb(246, 239, 232);
pub const MUTED: Color32 = Color32::from_rgb(197, 185, 193);
pub const BG: Color32 = Color32::from_rgb(24, 21, 29);
pub const PANEL: Color32 = Color32::from_rgb(83, 72, 79);
pub const PANEL_DEEP: Color32 = Color32::from_rgb(38, 31, 42);
pub const PANEL_EDGE: Color32 = Color32::from_rgb(120, 103, 111);
pub const HOT: Color32 = Color32::from_rgb(238, 53, 143);
pub const VIOLET: Color32 = Color32::from_rgb(124, 71, 177);
pub const LIME: Color32 = Color32::from_rgb(174, 231, 83);
pub const SKY: Color32 = Color32::from_rgb(104, 196, 229);

pub fn apply(ctx: &Context) {
    let mut visuals = Visuals::dark();
    visuals.panel_fill = PANEL;
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = PANEL_DEEP;
    visuals.faint_bg_color = Color32::from_rgb(57, 45, 57);
    visuals.selection.bg_fill = HOT;
    visuals.selection.stroke.color = INK;
    visuals.widgets.noninteractive.bg_fill = PANEL;
    visuals.widgets.noninteractive.fg_stroke.color = INK;
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(65, 52, 65);
    visuals.widgets.inactive.fg_stroke.color = INK;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, PANEL_EDGE);
    visuals.widgets.hovered.bg_fill = VIOLET;
    visuals.widgets.hovered.fg_stroke.color = INK;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, INK);
    visuals.widgets.active.bg_fill = HOT;
    visuals.widgets.active.fg_stroke.color = INK;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, INK);
    visuals.window_rounding = Rounding::same(4.0);
    visuals.menu_rounding = Rounding::same(3.0);
    visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 12.0,
        spread: 0.0,
        color: Color32::from_black_alpha(100),
    };
    let mut style = Style::default();
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(6.0, 6.0);
    style.spacing.button_padding = egui::vec2(9.0, 6.0);
    style.spacing.menu_margin = egui::Margin::same(4.0);
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(14.0, FontFamily::Monospace));
    style
        .text_styles
        .insert(TextStyle::Button, FontId::new(13.0, FontFamily::Monospace));
    style
        .text_styles
        .insert(TextStyle::Heading, FontId::new(22.0, FontFamily::Monospace));
    ctx.set_style(style);
}
