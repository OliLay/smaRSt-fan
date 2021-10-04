use sysfs_pwm::Pwm;

pub struct PwmControl {
    pwm: Pwm,
}

impl PwmControl {
    const SECOND_IN_NANOSECONDS: f64 = 1_000_000_000.0;

    pub fn new(
        initial_throttle_percentage: f64,
        pwm_chip: u32,
        pwm_channel: u32,
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

        fan_control.enable()?;
        fan_control.set_frequency(25000.0)?;
        fan_control.set_throttle(initial_throttle_percentage)?;

        Ok(fan_control)
    }

    pub fn enable(&self) -> Result<(), String> {
        match self.pwm.export() {
            Err(err) => return Err(format!("Could not export PWM. {}", err)),
            Ok(_) => {}
        };

        match self.pwm.enable(true) {
            Err(err) => return Err(format!("Could not enable PWM. {}", err)),
            Ok(_) => {}
        };

        Ok(())
    }

    pub fn disable(&self) -> Result<(), String> {
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
        self.pwm.enable(false).unwrap()
    }
}