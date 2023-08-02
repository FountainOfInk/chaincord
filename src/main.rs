#![allow(dead_code)]
#![allow(unused_variables)]

use eframe::{App, run_native, NativeOptions};
use egui::{CentralPanel, ScrollArea};
use reqwest::Client;
use serde::{Deserialize, de::DeserializeOwned};
use tokio::{self, runtime::Runtime};
use std::{fs, time::Duration};
use std::sync::mpsc::{Receiver, Sender, channel};


const BASE_API_URL: &str = "http://localhost:8000";
const USER_AGENT: &str = "reallycool https://amongus.rs, 942.3)";


struct AmongUs {
    susser: Susser,
    reciever: Receiver<String>,
    config: Config,
    current_messages: Vec<Message>
}

struct Susser {
    client: Client,
    transmitter: Sender<String>
}

#[derive(Deserialize)]
struct Message {
    author: String,
    contents: String
}

#[derive(Deserialize)]
struct Config {
    token: String
}

fn json_string_to_struct<T: DeserializeOwned>(json_string: &str) -> T {
    serde_json::from_str::<T>(&json_string).expect("Unable to deserialize string")
}
fn json_file_to_struct<T: DeserializeOwned>(path: &str) -> T {
    json_string_to_struct(&(fs::read_to_string(path).expect(&format!("Unable to read path: {}", path))))
}


impl AmongUs {
    fn set_dummy_messages_from_iter(&mut self, min: u32, max: u32) {
        let message_iter = (0..50).map(|i| Message {
            author: format!("Number {} among us fan", i),
            contents: format!("I love among us {} times more than anything else", i)
        });
        self.current_messages = Vec::from_iter(message_iter);
    }
    fn set_messages_from_api(&mut self, endpoint: &str) {
        send_api_request(self.susser.client.clone(), "/reallycoolmessages.json".to_owned(), "GET".to_owned(), self.susser.transmitter.clone());
        self.current_messages = json_string_to_struct(&self.reciever.recv().expect("Recieving message failed in set_messages_from_api"));
    }
}

impl App for AmongUs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let messages_area = ScrollArea::vertical().auto_shrink([false, false]);
            messages_area.show(ui, |ui| {
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
        let (transmitter, reciever) = channel::<String>();

        let mut app = AmongUs {
            susser: Susser {
                client: reqwest::Client::new(),
                transmitter
            },
            config: json_file_to_struct("config.json"),
            reciever,
            current_messages: vec![],
        };

        // app.current_messages = json_file_to_struct("messages.json");
        app.set_messages_from_api("/reallycoolmessages.json");

        app
    }
    
}
fn send_api_request(client: Client, endpoint: String, method: String, transmitter: Sender<String>) {
    let method = reqwest::Method::from_bytes(method.as_bytes()).expect("Invalid method type in send_api_response");
    tokio::spawn(
        async move {
            let response = client.request(method, format!("{}{}", BASE_API_URL, endpoint))
            .send().await
            .expect("Sending the request failed in send_api_request")
            .text().await.expect("Unable to get response text in send_api_reqest");

            transmitter.send(response).expect("Unable to send response string through channel in send_api_request");
        }
    );
    
}

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    run_native("amongus!!", NativeOptions::default(), Box::new(|cc| Box::new(AmongUs::new(cc)))).expect("its over");
}