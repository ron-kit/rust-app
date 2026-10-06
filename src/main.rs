// A minimal desktop app: a static window that shows an image.
//
// eframe = the "frame" that creates the OS window and runs the event loop.
// egui   = the immediate-mode GUI library that draws inside that window.

use eframe::egui;

fn main() -> eframe::Result {
    // Window settings. 620x471 is the penguin image's native size.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Tasker")
            .with_inner_size([2016.0, 1134.0]),
        ..Default::default()
    };

    // Opens the window and blocks until it is closed.
    eframe::run_native(
        "tasker",
        options,
        Box::new(|cc| {
            // Teach egui how to decode image bytes (JPEG, via the `image` crate).
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App))
        }),
    )
}

struct App;

impl eframe::App for App {
    // Called every frame. Immediate mode: we describe the whole UI each time.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE) // no padding/background around the image
            .show(ui, |ui| {
                // include_image! embeds the JPEG bytes into the binary at compile time,
                // so the app doesn't need the picture next to it at runtime.
                ui.add(egui::Image::new(egui::include_image!("../assets/biodome.jpeg")).fit_to_exact_size(ui.available_size()));
            });
    }
}
