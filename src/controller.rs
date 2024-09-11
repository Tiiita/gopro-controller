use std::error::Error;

use btleplug::platform::Manager;

async fn _discover() -> () {
    let manager = Manager::new().await.unwrap();

}
pub struct GoPro {
    pub name: String,
    pub recording: bool,
}

impl GoPro {
    pub fn new(name: String) -> Self {
        GoPro {
            name,
            recording: false,
        }
    }

    pub fn connect(&self) -> &Self {
        println!("Unimplemented");
        &self
    }

    pub fn disconnect(&self) {
        println!("Unimplemented");
    }

    pub fn record(&self) {}
    pub fn stop_record(&self) {}
}