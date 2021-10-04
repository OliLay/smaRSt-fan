use std::fmt;

#[derive(Copy, Clone)]
pub struct Status {
    pub rpm: Option<u64>,
    pub throttle: Option<f64>,
    pub temperature: Option<f64>,
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Throttle is {}%, RPM is {}, CPU temp is {}°C",
            self.option_to_string(self.throttle.map(|throttle| throttle * 100.)),
            self.option_to_string(self.rpm),
            self.option_to_string(self.temperature)
        )
    }
}

impl Status {
    fn option_to_string<T: ToString + fmt::Display>(&self, option: Option<T>) -> String {
        return option
            .map(|value| format!("{:.1}", value))
            .unwrap_or("N/A".into());
    }
}
