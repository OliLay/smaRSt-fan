use rppal::pwm::{Channel, Polarity, Pwm};

pub struct Control {
    pwm: Pwm,
}

impl Control {
    pub fn new(initial_speed_percentage: f64) -> Option<Control> {
        Control::new_with_channel(initial_speed_percentage, Channel::Pwm0)
    }

    pub fn new_with_channel(initial_speed_percentage: f64, channel: Channel) -> Option<Control> {
        let pwm = match Pwm::with_frequency(
            channel,
            25000.0,
            initial_speed_percentage,
            Polarity::Normal,
            true,
        ) {
            Ok(pwm) => pwm,
            Err(_) => return None,
        };

        Some(Control { pwm: pwm })
    }

    pub fn set_speed(&self, speed_percentage: f64) {
        self.pwm.set_duty_cycle(speed_percentage).unwrap();
    }

    pub fn get_speed(&self) -> f64 {
        self.pwm.duty_cycle().unwrap()
    }
}

impl Drop for Control {
    fn drop(&mut self) {
        self.pwm.disable().unwrap();
    }
}
