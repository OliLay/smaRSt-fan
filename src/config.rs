use config::*;
use glob::glob;
use log::LevelFilter;

#[derive(Debug)]
pub struct Config {
    // logging
    pub log_level: LevelFilter,
    // wiring
    pub tacho_gpio_pin: u64,
    pub pwm_chip: u32,
    pub pwm_channel: u32,
    // tacho
    pub tacho_enabled: bool,
    // control config
    pub target_temperature: f64,
    pub min_speed: f64,
    pub max_speed: f64,
    pub proportional: f64,
    pub derivative: f64,
    pub integral: f64,
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

        let get_value = |key: &str| match settings.get::<String>(key) {
            Ok(value) => value,
            Err(_) => panic!(
                "Config entry with key '{}' is not present in the config file. Please add it.",
                key
            ),
        };

        Config {
            log_level: get_value("logging.log_level").parse().unwrap(),
            tacho_gpio_pin: get_value("wiring.tacho_gpio_pin").parse().unwrap(),
            pwm_chip: get_value("wiring.pwm_chip").parse().unwrap(),
            pwm_channel: get_value("wiring.pwm_channel").parse().unwrap(),
            tacho_enabled: get_value("tacho.enabled").parse().unwrap(),
            target_temperature: get_value("control.target_temperature").parse().unwrap(),
            min_speed: get_value("control.constraints.min_speed").parse().unwrap(),
            max_speed: get_value("control.constraints.max_speed").parse().unwrap(),
            proportional: get_value("control.coefficients.proportional")
                .parse()
                .unwrap(),
            derivative: get_value("control.coefficients.derivative")
                .parse()
                .unwrap(),
            integral: get_value("control.coefficients.integral").parse().unwrap(),
        }
    }
}
