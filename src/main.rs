mod app;
mod model;
mod project;
mod runtime;
mod sprite;
mod theme;
mod tilemap;
mod ui;
mod world;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--play") {
        if let Some(path) = args.get(2) {
            let world = world::World::from_file(path.clone());
            return eframe::run_native(
                "Omasprite Player",
                eframe::NativeOptions::default(),
                Box::new(move |_| Ok(Box::new(world::Player(world)))),
            );
        }
    }
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
        Box::new(move |cc| {
            let mut app = app::StudioApp::new(cc);
            if args.get(1).map(String::as_str) == Some("--project") {
                if let Some(path) = args.get(2) {
                    app.open_project(path.clone());
                }
            }
            Ok(Box::new(app))
        }),
    )
}
