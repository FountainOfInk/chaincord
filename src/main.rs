#![allow(dead_code)]
#![allow(unused_variables)]

use eframe::{App, run_native, NativeOptions};
use egui::{CentralPanel, ScrollArea};
use serde::{Deserialize, Serialize};
use std::fs;

fn main() {
    run_native("amongus!!", NativeOptions::default(), Box::new(|cc| Box::new(AmongUs::new(cc)))).expect("its over");
}

struct AmongUs {
    current_messages: Vec<Message>
}

impl AmongUs {
    fn initialize_from_json_file(file_name: String) -> Self {
        let messagejsondata = fs::read_to_string(file_name).expect("Unable to read messages.json");
        let deserialized: Vec<Message> = serde_json::from_str(&messagejsondata).unwrap();
        AmongUs { current_messages: Vec::from_iter(deserialized)}
    }
    fn initialize_from_iter(min: u32, max: u32) -> Self {
        let message_iter = (0..50).map(|i| Message {
            author: format!("Number {} among us fan", i),
            contents: format!("I love among us {} times more than anything else", i)
        });
        AmongUs { current_messages: Vec::from_iter(message_iter) }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    author: String,
    contents: String
}

impl App for AmongUs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for msg in &self.current_messages {
                    ui.label(format!("{}:\n{}", msg.author, msg.contents));
                }
            })
        });
    }
}

impl AmongUs {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        AmongUs::initialize_from_iter(0,50)
    }
}