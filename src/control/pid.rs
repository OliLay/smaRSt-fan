use pid::Pid;
use crate::config::Config;

pub struct PidControl {
    pid: Pid<f64>,
    min_throttle: f64,
    max_throttle: f64,
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
            min_throttle: config.min_throttle,
            max_throttle: config.max_throttle,
        }
    }

    pub fn control(&mut self, current_temperature: f64, current_throttle: f64) -> f64 {
        let gain = self.pid.next_control_output(current_temperature).output;

        f64::max(
            self.min_throttle,
            f64::min(self.max_throttle, current_throttle - gain),
        )
    }
}