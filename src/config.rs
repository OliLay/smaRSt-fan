use config::*;
use glob::glob;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    // wiring
    pub tacho_gpio_pin: u8,
    // control config
    pub target_temperature: f64,
    pub min_speed: f64,
    pub max_speed: f64,
}

impl Config {
    pub fn new() -> Config {
        let mut settings = config::Config::default();
        settings
            .merge(
                glob("/etc/smaRSt-fan/*")
                    .unwrap()
                    .map(|path| File::from(path.unwrap()))
                    .collect::<Vec<_>>(),
            )
            .unwrap();

        let config_map = settings.try_into::<HashMap<String, String>>().unwrap();

        Config {
            tacho_gpio_pin: config_map["tacho_gpio_pin"].parse().unwrap(),
            target_temperature: config_map["target_temperature"].parse().unwrap(),
            min_speed: config_map["min_speed"].parse().unwrap(),
            max_speed: config_map["max_speed"].parse().unwrap(),
        }
    }
}
