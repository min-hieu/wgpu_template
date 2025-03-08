mod module;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lichthaus",
        native_options,
        Box::new(|cc| Ok(Box::new(module::app::MainApp::new(cc)))),
    );
}

