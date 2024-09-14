use wifi_rs::{self, prelude::{Config, Connectivity, WifiConnectionError}, WiFi};
use wifiscanner::Wifi;

pub fn init() -> WiFi {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    WiFi::new(config)
}

pub struct GoPro {
    pub recording: bool,
    pub mode: Option<GoProMode>,
    pub wifi_info: Wifi,
}

impl<'a> GoPro {
    pub fn new( wifi_info: Wifi) -> Self {
        GoPro {
            recording: false,
            mode: None,
            wifi_info,
        }
    }

    pub fn connect(&self, mut wifi: WiFi, password: &'a str) -> Result<bool, WifiConnectionError> {
        wifi.connect(&self.wifi_info.ssid, &password)
    }

    pub fn send_command(_command: GoProCommand) {
        todo!("");
    }

    pub fn get_info(_info: GoProInfo) -> Result<(), ()> {
        todo!("");
    } 
}

pub enum GoProCommand {
    RecordStart,
    RecordStop,
    ChangeMode(GoProMode),
}

pub enum GoProInfo {
    Recording(bool),

    //The battery status, where the u8 represents 0-100%
    BatteryStatus(u8),
}


pub enum GoProMode {
    Photo,
    Record,
    Timelaps,
}