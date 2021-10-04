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
    pub min_throttle: f64,
    pub max_throttle: f64,
    pub proportional: f64,
    pub derivative: f64,
    pub integral: f64,
    // mqtt
    pub mqtt_enabled: bool,
    pub mqtt_broker_address: String,
    pub mqtt_broker_port: u16,
    pub mqtt_topic_rpm: String,
    pub mqtt_topic_throttle: String,
    pub mqtt_topic_temperature: String,
    pub mqtt_period_secs: u64,
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
            min_throttle: get_value("control.constraints.min_throttle").parse().unwrap(),
            max_throttle: get_value("control.constraints.max_throttle").parse().unwrap(),
            proportional: get_value("control.coefficients.proportional")
                .parse()
                .unwrap(),
            derivative: get_value("control.coefficients.derivative")
                .parse()
                .unwrap(),
            integral: get_value("control.coefficients.integral").parse().unwrap(),
            mqtt_enabled: get_value("mqtt.enabled").parse().unwrap(),
            mqtt_broker_address: get_value("mqtt.broker.address").parse().unwrap(),
            mqtt_broker_port: get_value("mqtt.broker.port").parse().unwrap(),
            mqtt_topic_rpm: get_value("mqtt.topics.rpm").parse().unwrap(),
            mqtt_topic_throttle: get_value("mqtt.topics.throttle").parse().unwrap(),
            mqtt_topic_temperature: get_value("mqtt.topics.temperature").parse().unwrap(),
            mqtt_period_secs: get_value("mqtt.period_secs").parse().unwrap(),
        }
    }
}
