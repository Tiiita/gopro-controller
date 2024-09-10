fn scan() -> Vec<GoPro> {
    todo!("Unimplemented");
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
