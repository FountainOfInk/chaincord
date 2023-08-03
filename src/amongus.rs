pub mod susser;

use std::sync::mpsc::{channel, Sender};
use eframe::App;
use egui::{CentralPanel, ScrollArea};
use serde_json::Value;
use crate::utils::json_file_to_struct;
use crate::config::Config;
use crate::discord_types::{User, Message, ApiError};
use crate::utils::json_string_to_struct;
use susser::Susser;

use reqwest::{self, Method, header::HeaderMap};

const BASE_API_URL: &str = "https://discord.com/api/v10/";


pub struct AmongUs {
        susser: Susser,
        current_messages: Vec<Message>
}


impl AmongUs {
        fn set_dummy_messages_from_iter(&mut self, min: u32, max: u32) {
            let message_iter = (min..max).map(|i| Message {
                author: User{
                    username: format!("Number {} among us fan", i),
                    id: "432".to_owned()
                },
                content: format!("I love among us {} times more than anything else", i)
            });
            self.current_messages = Vec::from_iter(message_iter);
        }

        fn set_messages_from_api(&mut self, endpoint: &str) {
            let (transmitter, reciever) = channel::<Result<String, ApiError>>();
            self.send_api_request(endpoint.to_owned(), "GET".to_owned(), transmitter);
            let response = reciever.recv().expect("Recieving message failed in set_messages_from_api");
            let mut current_messages = match response {
                Ok(respbody) => json_string_to_struct::<Vec<Message>>(&respbody).expect("Couldn't deserialize data"),
                Err(error) => { 
                    self.handle_err(&error);
                }
            };
            current_messages.reverse();
            self.current_messages = current_messages;
        }
        
        fn send_api_request(&self, endpoint: String, method: String, transmitter: Sender<Result<String, ApiError>>) {
                let method = Method::from_bytes(method.as_bytes()).expect("Invalid method type in send_api_response");
                let client = self.susser.client.clone();
                tokio::spawn(
                    async move {
                        let http_response = client.request(method, format!("{}{}", BASE_API_URL, endpoint))
                        .send().await
                        .expect("Sending the request failed in send_api_request");
                        
                        let status = http_response.status();
                        let response_body: String = http_response.text().await.expect("could not decode response");
                        println!("{}", response_body);
                        
                        let parsed_response: Result<String, ApiError>;
                        if status.is_client_error() || status.is_server_error() {
                            parsed_response = Err(json_string_to_struct::<ApiError>(&response_body)
                            .expect("Couldn't parse response with failure code as ApiError"));
                        } else {
                            parsed_response = Ok(response_body);
                        }
                        transmitter.send(parsed_response).expect("Unable to send response string through channel in send_api_request");
                    }
                );
                
        }

        fn handle_err(&self, error: &ApiError) -> ! {
            panic!("Api responded with error code {},\nmessage:\n{}", error.code, error.message);
        }

}
    

    
impl AmongUs {
        /// Called once before the first frame.
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
                let config: Config = json_file_to_struct("config.json").expect("Couldn't deserialize config");

                let client = reqwest::Client::builder();
                let mut headers = HeaderMap::new();
                headers.insert("Authorization", format!("{}{}", "Bot ", config.token).parse().unwrap());
                let client = client.user_agent(config.user_agent);
                let client = client.default_headers(headers);
                let client = client.build().expect("Failed to build client");
                
                let mut app = AmongUs {
                susser: Susser {
                        client
                },
                current_messages: vec![],
                };
        
                // app.current_messages = json_file_to_struct("");
                app.set_messages_from_api("channels/1234/messages");
        
                app
        }                
}

impl App for AmongUs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let messages_area = ScrollArea::vertical().auto_shrink([false, false]);
            messages_area.show(ui, |ui| {
                for msg in &self.current_messages {
                    ui.label(format!("{}:\n{}", msg.author.username, msg.content));
                }
            })
        });
    }
}