pub mod config;
pub mod control;
pub mod cpu_temp;
pub mod logging;
pub mod signals;
pub mod tacho;

use crate::config::Config;
use control::FanControl;
use control::PidControl;
use cpu_temp::CpuTemperatureReader;
use logging::initialize_logging;
use signals::SignalHandler;
use tacho::Tacho;

use log::{info, trace, warn};
use std::time::Duration;

fn main() {
    let signal_handler = SignalHandler::new().unwrap();

    let config = Config::new();
    initialize_logging(config.log_level);
    info!("Config is {:?}", config);

    let tacho: Option<Tacho> = if config.tacho_enabled {
        let mut tacho = Tacho::new(config.tacho_gpio_pin);
        tacho.start();
        Some(tacho)
    } else {
        None
    };

    let fan_control = FanControl::new(0.0, config.pwm_chip, config.pwm_channel).unwrap();
    let mut pid_control = PidControl::new(&config);
    let cpu_temp_reader = CpuTemperatureReader::new();

    while !signal_handler.should_terminate() {
        let current_temperature = cpu_temp_reader.get_temperature().unwrap();
        let current_rpm = if tacho.is_some() {
            tacho.as_ref().unwrap().get_rpm()
        } else {
            None
        };
        let current_speed = fan_control.get_speed().unwrap();

        trace!(
            "Speed is {:.1}%, RPM is {}, CPU temp is {:.1}°C",
            current_speed * 100.0,
            current_rpm
                .map(|rpm| rpm.to_string())
                .unwrap_or("N/A".into()),
            current_temperature
        );

        let new_speed = pid_control.control(current_temperature, current_speed);
        match fan_control.set_speed(new_speed) {
            Err(err) => warn!("Could not set fan speed! {}", err),
            _ => {}
        }
        std::thread::sleep(Duration::from_millis(300));
    }

    warn!("Received exit signal. Terminating.")
}
