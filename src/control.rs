use pid::Pid;
use rppal::pwm::{Channel, Polarity, Pwm};

pub struct PidControl {
    pid: Pid<f64>,
    min_speed: f64,
    max_speed: f64,
}

impl PidControl {
    pub fn new(desired_temperature: f64, min_speed: f64, max_speed: f64) -> PidControl {
        let pid = Pid::new(0.001, 0.001, 0.0, 0.01, 0.01, 0.001, desired_temperature);

        PidControl {
            pid: pid,
            min_speed: min_speed,
            max_speed: max_speed,
        }
    }

    pub fn control(&mut self, current_temperature: f64, current_speed: f64) -> f64 {
        let gain = self.pid.next_control_output(current_temperature).output;

        f64::max(
            self.min_speed,
            f64::min(self.max_speed, current_speed - gain),
        )
    }
}

pub struct FanControl {
    pwm: Pwm,
}

impl FanControl {
    pub fn new(initial_speed_percentage: f64) -> Option<FanControl> {
        FanControl::new_with_channel(initial_speed_percentage, Channel::Pwm0)
    }

    pub fn new_with_channel(initial_speed_percentage: f64, channel: Channel) -> Option<FanControl> {
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

        Some(FanControl { pwm: pwm })
    }

    pub fn set_speed(&self, speed_percentage: f64) {
        self.pwm.set_duty_cycle(speed_percentage).unwrap();
    }

    pub fn get_speed(&self) -> Option<f64> {
        match self.pwm.duty_cycle() {
            Ok(speed) => Some(speed),
            Err(_) => None,
        }
    }
}

impl Drop for FanControl {
    fn drop(&mut self) {
        self.pwm.disable().unwrap();
    }
}
