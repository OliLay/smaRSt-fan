use systemstat::{System, Platform};

pub struct CpuTemperatureReader {
    system: System
}

impl CpuTemperatureReader {

    pub fn new() -> CpuTemperatureReader {
        let system = System::new();
        CpuTemperatureReader{system: system}
    }

    pub fn get_temperature(&self) -> Option<f64> {
        match self.system.cpu_temp() {
            Ok(temp) => Some(temp.into()),
            Err(_) => None
        }
    }
}