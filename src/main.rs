pub mod config;
pub mod control;
pub mod cpu_temp;
pub mod logging;
pub mod mqtt;
pub mod signals;
pub mod status;
pub mod tacho;

use crate::config::Config;
use control::FanControl;
use control::PidControl;
use cpu_temp::CpuTemperatureReader;
use logging::initialize_logging;
use mqtt::MqttClient;
use signals::SignalHandler;
use status::Status;
use tacho::Tacho;

use log::{info, trace, warn};
use parking_lot::Mutex;
use std::sync::Arc;
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

    let status = Arc::new(Mutex::new(Status {
        rpm: None,
        speed: None,
        temperature: None,
    }));

    let fan_control = FanControl::new(0.0, config.pwm_chip, config.pwm_channel).unwrap();

    // TODO: configure from config
    let mqtt_client = MqttClient::new(status.clone());
    mqtt_client.start();

    let mut pid_control = PidControl::new(&config);
    let cpu_temp_reader = CpuTemperatureReader::new();

    while !signal_handler.should_terminate() {
        let current_temperature = cpu_temp_reader.get_temperature();
        let current_speed = fan_control.get_speed();
        let current_rpm = if tacho.is_some() {
            tacho.as_ref().unwrap().get_rpm()
        } else {
            None
        };

        let new_speed = pid_control.control(current_temperature.unwrap(), current_speed.unwrap());

        match fan_control.set_speed(new_speed) {
            Err(err) => warn!("Could not set fan speed! {}", err),
            _ => {}
        }

        {
            let mut aquired_status = status.lock();
            aquired_status.rpm = current_rpm;
            aquired_status.speed = current_speed;
            aquired_status.temperature = current_temperature;

            trace!("{}", aquired_status);
        }

        std::thread::sleep(Duration::from_millis(300));
    }

    info!("Received exit signal. Terminating.")
}
