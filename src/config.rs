use config::*;
use glob::glob;
use log::LevelFilter;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    // logging
    pub log_level: LevelFilter,
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

        fn get_value<'a>(map: &'a HashMap<String, String>, key: &'a str) -> &'a str {
            match map.get(key) {
                Some(value) => value,
                None => panic!(
                    "Config entry with key '{}' is not present in the config file. Please add it.",
                    key
                ),
            }
        }

        Config {
            log_level: get_value(&config_map, "log_level").parse().unwrap(),
            tacho_gpio_pin: get_value(&config_map, "tacho_gpio_pin").parse().unwrap(),
            target_temperature: get_value(&config_map, "target_temperature")
                .parse()
                .unwrap(),
            min_speed: get_value(&config_map, "min_speed").parse().unwrap(),
            max_speed: get_value(&config_map, "max_speed").parse().unwrap(),
        }
    }
}
