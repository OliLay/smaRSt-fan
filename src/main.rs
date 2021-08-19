pub mod tacho;
pub mod control;

use tacho::Tacho;
use control::Control;

use std::time::Duration;

const TACHO_GPIO_PIN: u8 = 5;

fn main() {
    let mut tacho: Tacho = Tacho::new(TACHO_GPIO_PIN);
    tacho.start();

    let control = Control::new(1.0).unwrap();

    loop {
        println!(
            "Speed is {}%, RPM is {}",
            control.get_speed() * 100.0,
            tacho
                .get_rpm()
                .map(|rpm| rpm.to_string())
                .unwrap_or("N/A".into())
        );
        std::thread::sleep(Duration::from_secs(1));
    }

    tacho.stop();
}
