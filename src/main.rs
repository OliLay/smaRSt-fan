pub mod tacho;
pub mod control;
pub mod cpu_temp;
pub mod signals;

use tacho::Tacho;
use control::Control;
use cpu_temp::CpuTemperatureReader;
use signals::SignalHandler;

use std::time::Duration;

const TACHO_GPIO_PIN: u8 = 5;

fn main() {
    let signal_handler = SignalHandler::new().unwrap();

    let mut tacho: Tacho = Tacho::new(TACHO_GPIO_PIN);
    tacho.start();
    let control = Control::new(1.0).unwrap();
    let cpu_temp_reader = CpuTemperatureReader::new();

    while !signal_handler.should_terminate() {
        println!(
            "Speed is {}%, RPM is {}, CPU temp is {:.1}°C",
            control.get_speed().unwrap() * 100.0,
            tacho
                .get_rpm()
                .map(|rpm| rpm.to_string())
                .unwrap_or("N/A".into()),
            cpu_temp_reader.get_temperature().unwrap()
        );
        std::thread::sleep(Duration::from_secs(1));
    }
}
