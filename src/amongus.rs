pub mod susser;

use std::sync::mpsc::{channel, Sender};
use eframe::App;
use egui::{CentralPanel, ScrollArea};
use crate::utils::json_file_to_struct;
use crate::config::Config;
use crate::message::Message;
use crate::utils::json_string_to_struct;
use susser::Susser;

use reqwest::{self, Method};

const BASE_API_URL: &str = "http://localhost:8000";
const USER_AGENT: &str = "reallycool https://amongus.rs, 942.3)";


pub struct AmongUs {
        susser: Susser,
        config: Config,
        current_messages: Vec<Message>
}


impl AmongUs {
        fn set_dummy_messages_from_iter(&mut self, min: u32, max: u32) {
            let message_iter = (min..max).map(|i| Message {
                author: format!("Number {} among us fan", i),
                contents: format!("I love among us {} times more than anything else", i)
            });
            self.current_messages = Vec::from_iter(message_iter);
        }

        fn set_messages_from_api(&mut self, endpoint: &str) {
            let (transmitter, reciever) = channel::<String>();
            self.send_api_request("/reallycoolmessages.json".to_owned(), "GET".to_owned(), transmitter);
            self.current_messages = json_string_to_struct(&reciever.recv().expect("Recieving message failed in set_messages_from_api"));
        }
        
        fn send_api_request(&self, endpoint: String, method: String, transmitter: Sender<String>) {
                let method = Method::from_bytes(method.as_bytes()).expect("Invalid method type in send_api_response");
                let client = self.susser.client.clone();
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
}
    

    
impl AmongUs {
        /// Called once before the first frame.
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
                let mut app = AmongUs {
                susser: Susser {
                        client: reqwest::Client::new(),
                },
                config: json_file_to_struct("config.json"),
                current_messages: vec![],
                };
        
                // app.current_messages = json_file_to_struct("messages.json");
                app.set_messages_from_api("/reallycoolmessages.json");
        
                app
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