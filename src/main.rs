use eframe::{App, run_native, NativeOptions};
use egui::CentralPanel;

fn main() {
    run_native("amongus!!", NativeOptions::default(), Box::new(|cc| Box::new(AmongUs::new(cc))));
}

#[derive(Default)]
struct AmongUs;

impl App for AmongUs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("ammmmfcgfdxvfrd")
        });
    }
}

impl AmongUs {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}