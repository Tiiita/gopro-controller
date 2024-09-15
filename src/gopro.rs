use tokio_wifiscanner::Wifi as WifiInfo;
use wifi_rs::{
    self,
    prelude::{Config, Connectivity, WifiConnectionError},
    WiFi,
};

pub fn init_connector() -> WiFi {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    WiFi::new(config)
}

pub struct GoPro {
    pub recording: bool,
    pub mode: Option<GoProMode>,
    pub wifi_info: WifiInfo,
    pub connector: WiFi,
}

impl<'a> GoPro {
    pub fn new(wifi_info: WifiInfo) -> Self {
        GoPro {
            recording: false,
            mode: None,
            wifi_info,
            connector: init_connector(),
        }
    }

    pub fn connect(&mut self, password: &'a str) -> Result<bool, WifiConnectionError> {
        self.connector.connect(&self.wifi_info.ssid, &password)
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
