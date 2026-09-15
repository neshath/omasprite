mod app;
mod model;
mod theme;
mod ui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("OMASPRITE")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1100.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "OMASPRITE",
        options,
        Box::new(|cc| Ok(Box::new(app::StudioApp::new(cc)))),
    )
}
