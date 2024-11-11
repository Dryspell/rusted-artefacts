mod app;
mod requests;

use app::ClientApp;
use dotenv::dotenv;
use eframe::NativeOptions;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from `.env`

    // Run the GUI application and handle potential errors
    eframe::run_native(
        "API Request GUI",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ClientApp::default()))),
    )
    .expect("Failed to run eframe");
}
