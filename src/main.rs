pub mod config;
pub mod control;
pub mod cpu_temp;
pub mod signals;
pub mod tacho;
pub mod logging;

use crate::config::Config;
use control::FanControl;
use control::PidControl;
use cpu_temp::CpuTemperatureReader;
use signals::SignalHandler;
use tacho::Tacho;
use logging::initialize_logging;

use std::time::Duration;
use log::{info, warn};


fn main() {
    let signal_handler = SignalHandler::new().unwrap();

    let config = Config::new();
    initialize_logging(config.log_level);
    info!("Config is {:?}", config);

    let mut tacho: Tacho = Tacho::new(config.tacho_gpio_pin);
    tacho.start();
    let fan_control = FanControl::new(0.0).unwrap();
    let mut pid_control = PidControl::new(
        config.target_temperature,
        config.min_speed,
        config.max_speed,
    );
    let cpu_temp_reader = CpuTemperatureReader::new();

    while !signal_handler.should_terminate() {
        let current_temperature = cpu_temp_reader.get_temperature().unwrap();
        let current_rpm = tacho.get_rpm();
        let current_speed = fan_control.get_speed().unwrap();

        info!(
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

    warn!("Received exit signal. Terminating.")
}
