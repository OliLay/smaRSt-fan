pub mod config;
pub mod control;
pub mod logging;
pub mod mqtt;
pub mod sensing;
pub mod signals;

use crate::config::Config;
use control::pid::PidControl;
use control::pwm::PwmControl;
use log::{info, trace, warn};
use logging::initialize_logging;
use mqtt::MqttClient;
use parking_lot::Mutex;
use sensing::cpu_temp::CpuTemperatureReader;
use sensing::status::Status;
use sensing::tacho::Tacho;
use signals::SignalHandler;
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
        throttle: None,
        temperature: None,
    }));

    let fan_control = PwmControl::new_from_config(&config).unwrap();

    if config.mqtt_enabled {
        let mqtt_client = MqttClient::new(status.clone(), &config);
        mqtt_client.start();
    }

    let mut pid_control = PidControl::new(&config);
    let cpu_temp_reader = CpuTemperatureReader::new();

    while !signal_handler.should_terminate() {
        let current_temperature = cpu_temp_reader.get_temperature();
        let current_throttle = fan_control.get_throttle();
        let current_rpm = if tacho.is_some() {
            tacho.as_ref().unwrap().get_rpm()
        } else {
            None
        };

        let new_throttle =
            pid_control.control(current_temperature.unwrap(), current_throttle.unwrap());

        match fan_control.set_throttle(new_throttle) {
            Err(err) => warn!("Could not set fan throttle! {}", err),
            _ => {}
        }

        {
            let mut aquired_status = status.lock();
            aquired_status.rpm = current_rpm;
            aquired_status.throttle = current_throttle;
            aquired_status.temperature = current_temperature;

            trace!("{}", aquired_status);
        }

        std::thread::sleep(Duration::from_millis(300));
    }

    info!("Received exit signal. Terminating.")
}
