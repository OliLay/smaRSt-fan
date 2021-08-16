pub mod tacho;

use tacho::Tacho;

use rppal::pwm::{Channel, Pwm, Polarity};
use std::time::Duration;

const TACHO_GPIO_PIN: u8 = 6;

fn main() {
    let mut tacho : Tacho = Tacho::new(TACHO_GPIO_PIN);
    tacho.start();

    // TODO: write PWM/Control abstraction.
    let pwm = Pwm::with_frequency(Channel::Pwm0, 25000.0, 0.1, Polarity::Normal, true).unwrap();

    loop {
        pwm.set_duty_cycle(0.2).unwrap();
        println!("Duty cycle is {}, RPM is {}", pwm.duty_cycle().unwrap(), tacho.get_rpm().unwrap_or(0));
        std::thread::sleep(Duration::from_secs(1));
    }

    tacho.stop();
}
