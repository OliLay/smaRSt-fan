use crate::config::Config;
use pid::Pid;
use sysfs_pwm::Pwm;

pub struct PidControl {
    pid: Pid<f64>,
    min_speed: f64,
    max_speed: f64,
}

impl PidControl {
    pub fn new(config: &Config) -> PidControl {
        let pid = Pid::new(
            config.proportional,
            config.integral,
            config.derivative,
            0.01,
            0.01,
            0.01,
            config.target_temperature,
        );

        PidControl {
            pid: pid,
            min_speed: config.min_speed,
            max_speed: config.max_speed,
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

// TODO: error handling (return results maybe? and handle in logic then instead of unwrap())
impl FanControl {
    const SECOND_IN_NANOSECONDS: f64 = 1000000000.0;

    pub fn new(
        initial_speed_percentage: f64,
        pwm_chip: u32,
        pwm_channel: u32,
    ) -> Option<FanControl> {
        let pwm = match Pwm::new(pwm_chip, pwm_channel) {
            Ok(pwm) => pwm,
            Err(_) => return None,
        };

        let fan_control = FanControl { pwm: pwm };

        fan_control.enable();
        fan_control.set_frequency(25000.0);
        fan_control.set_speed(initial_speed_percentage);

        Some(fan_control)
    }

    pub fn enable(&self) {
        self.pwm.enable(true).unwrap();
    }

    pub fn disable(&self) {
        self.pwm.enable(false).unwrap();
    }

    pub fn set_speed(&self, speed_percentage: f64) {
        let duty_cycle_ns =
            (self.pwm.get_period_ns().unwrap() as f64 * speed_percentage).round() as u32;
        self.pwm.set_duty_cycle_ns(duty_cycle_ns).unwrap();
    }

    pub fn get_speed(&self) -> Option<f64> {
        match self.pwm.get_duty_cycle_ns() {
            Ok(duty_cycle_ns) => Some(duty_cycle_ns as f64 / Self::SECOND_IN_NANOSECONDS),
            Err(_) => None,
        }
    }

    pub fn set_frequency(&self, frequency: f64) {
        let period_ns = ((1.0 / frequency) * Self::SECOND_IN_NANOSECONDS) as u32;
        self.pwm.set_period_ns(period_ns).unwrap();
    }
}

impl Drop for FanControl {
    fn drop(&mut self) {
        self.pwm.enable(false).unwrap()
    }
}
