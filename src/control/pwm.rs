use crate::config::Config;
use retry::{delay::Fixed, retry};
use sysfs_pwm::Pwm;

pub struct PwmControl {
    pwm: Pwm,
}

impl PwmControl {
    const SECOND_IN_NANOSECONDS: f64 = 1_000_000_000.0;

    pub fn new_from_config(config: &Config) -> Result<PwmControl, String> {
        PwmControl::new(
            0.0,
            config.pwm_chip,
            config.pwm_channel,
            config.pwm_frequency,
        )
    }

    pub fn new(
        initial_throttle_percentage: f64,
        pwm_chip: u32,
        pwm_channel: u32,
        pwm_frequency: f64,
    ) -> Result<PwmControl, String> {
        let pwm = match Pwm::new(pwm_chip, pwm_channel) {
            Ok(pwm) => pwm,
            Err(_) => {
                return Err(format!(
                    "Could not create PWM control for chip {} on channel {}",
                    pwm_chip, pwm_channel
                ))
            }
        };

        let fan_control = PwmControl { pwm: pwm };

        fan_control.export()?;

        // retry is needed here, as exporting takes a while,
        // and the method above is not blocking.
        match retry(Fixed::from_millis(300), || {
            fan_control.set_frequency(pwm_frequency)?;
            fan_control.set_throttle(initial_throttle_percentage)?;
            fan_control.enable()?;

            Ok(())
        }) {
            Err(retry::Error::Operation {
                error,
                total_delay: _,
                tries: _,
            }) => Err(error),
            _ => Ok(fan_control),
        }
    }

    fn export(&self) -> Result<(), String> {
        match self.pwm.export() {
            Err(err) => Err(format!("Could not export PWM. {}", err)),
            Ok(_) => Ok(()),
        }
    }

    pub fn enable(&self) -> Result<(), String> {
        match self.pwm.enable(true) {
            Err(err) => Err(format!("Could not enable PWM. {}", err)),
            Ok(_) => Ok(()),
        }
    }

    pub fn destroy(&self) -> Result<(), String> {
        match self.pwm.enable(false) {
            Err(err) => return Err(format!("Could not disable PWM. {}", err)),
            Ok(_) => {}
        };

        match self.pwm.unexport() {
            Err(err) => return Err(format!("Could not unexport PWM. {}", err)),
            Ok(_) => {}
        };

        Ok(())
    }

    pub fn set_throttle(&self, throttle_percentage: f64) -> Result<(), String> {
        let duty_cycle = match self.pwm.get_period_ns() {
            Ok(period) => (period as f64 * throttle_percentage).round() as u32,
            Err(err) => return Err(format!("Could not get current period from PWM. {}", err)),
        };

        match self.pwm.set_duty_cycle_ns(duty_cycle) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Could not set duty cycle on PWM. {}", err)),
        }
    }

    pub fn get_throttle(&self) -> Option<f64> {
        let period = self.pwm.get_period_ns();
        let duty_cycle = self.pwm.get_duty_cycle_ns();

        match period {
            Ok(period) => match duty_cycle {
                Ok(duty_cycle) => Some(duty_cycle as f64 / period as f64),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn set_frequency(&self, frequency: f64) -> Result<(), String> {
        let period_ns = ((1.0 / frequency) * Self::SECOND_IN_NANOSECONDS) as u32;
        match self.pwm.set_period_ns(period_ns) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Could not set period on PWM. {}", err)),
        }
    }
}

impl Drop for PwmControl {
    fn drop(&mut self) {
        self.destroy().unwrap();
    }
}
