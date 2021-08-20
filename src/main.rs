pub mod tacho;
pub mod control;
pub mod cpu_temp;
pub mod signals;

use tacho::Tacho;
use control::FanControl;
use control::PidControl;
use cpu_temp::CpuTemperatureReader;
use signals::SignalHandler;


use std::time::Duration;

const TACHO_GPIO_PIN: u8 = 5;

fn main() {
    let signal_handler = SignalHandler::new().unwrap();

    let mut tacho: Tacho = Tacho::new(TACHO_GPIO_PIN);
    tacho.start();
    let fan_control = FanControl::new(0.5).unwrap();
    let mut pid_control = PidControl::new(45.0);
    let cpu_temp_reader = CpuTemperatureReader::new();

    while !signal_handler.should_terminate() {
        let current_temperature = cpu_temp_reader.get_temperature().unwrap();
        let current_rpm = tacho.get_rpm();
        let current_speed = fan_control.get_speed().unwrap();

        println!(
            "Speed is {:.1}%, RPM is {}, CPU temp is {:.1}°C",
            current_speed * 100.0,
            current_rpm
                .map(|rpm| rpm.to_string())
                .unwrap_or("N/A".into()),
            current_temperature
        );
        
        let new_speed = pid_control.control(current_temperature, current_speed);
        fan_control.set_speed(new_speed);
        
        std::thread::sleep(Duration::from_millis(300));
    }
}
