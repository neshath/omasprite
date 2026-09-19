use egui::{Color32, Context, FontFamily, FontId, Rounding, Style, TextStyle, Visuals};

pub const INK: Color32 = Color32::from_rgb(245, 245, 247);
pub const MUTED: Color32 = Color32::from_rgb(161, 161, 170);
pub const BG: Color32 = Color32::from_rgb(17, 17, 20);
pub const PANEL: Color32 = Color32::from_rgb(29, 29, 33);
pub const PANEL_DEEP: Color32 = Color32::from_rgb(22, 22, 25);
pub const PANEL_RAISED: Color32 = Color32::from_rgb(40, 40, 46);
pub const PANEL_EDGE: Color32 = Color32::from_rgb(61, 61, 68);
pub const HOT: Color32 = Color32::from_rgb(255, 56, 153);
pub const HOT_SOFT: Color32 = Color32::from_rgba_premultiplied(255, 56, 153, 42);
pub const VIOLET: Color32 = Color32::from_rgb(128, 96, 212);
pub const LIME: Color32 = Color32::from_rgb(167, 222, 103);
pub const SKY: Color32 = Color32::from_rgb(100, 188, 224);

pub fn apply(ctx: &Context) {
    let mut visuals = Visuals::dark();
    visuals.panel_fill = PANEL;
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = PANEL_DEEP;
    visuals.faint_bg_color = Color32::from_rgb(38, 38, 43);
    visuals.selection.bg_fill = HOT_SOFT;
    visuals.selection.stroke.color = INK;
    visuals.widgets.noninteractive.bg_fill = PANEL;
    visuals.widgets.noninteractive.fg_stroke.color = INK;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.fg_stroke.color = INK;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
    visuals.widgets.hovered.bg_fill = PANEL_RAISED;
    visuals.widgets.hovered.fg_stroke.color = INK;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, PANEL_EDGE);
    visuals.widgets.active.bg_fill = HOT;
    visuals.widgets.active.fg_stroke.color = INK;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, INK);
    visuals.window_rounding = Rounding::same(10.0);
    visuals.menu_rounding = Rounding::same(8.0);
    visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 12.0,
        spread: 0.0,
        color: Color32::from_black_alpha(100),
    };
    let mut style = Style::default();
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);
    style.spacing.menu_margin = egui::Margin::same(4.0);
    style.spacing.interact_size = egui::vec2(40.0, 28.0);
    style.spacing.slider_width = 128.0;
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(14.0, FontFamily::Proportional));
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(13.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(21.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(12.0, FontFamily::Proportional),
    );
    ctx.set_style(style);
}
