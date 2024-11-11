use eframe::egui;
use std::sync::{Arc, Mutex};
use tokio::spawn;

use crate::requests::pokemon_test_request;

pub struct ClientApp {
    pub api_response: Arc<Mutex<String>>,
}

impl Default for ClientApp {
    fn default() -> Self {
        Self {
            api_response: Arc::new(Mutex::new("Click the button to make a request".to_owned())),
        }
    }
}

impl eframe::App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("API Request Example");

            // Display JSON response in a scrollable text box
            egui::ScrollArea::vertical().show(ui, |ui| {
                let response_text = self.api_response.lock().unwrap().clone();
                ui.label(&response_text);
            });

            // Button to trigger API request
            if ui.button("Make API Request").clicked() {
                let api_response = Arc::clone(&self.api_response);
                spawn(async move {
                    let response_text = pokemon_test_request().await.unwrap_or_else(|_| "Request failed".to_string());
                    *api_response.lock().unwrap() = response_text;
                });
            }
        });
    }
}
