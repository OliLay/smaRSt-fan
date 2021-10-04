use log::{error, warn};
use parking_lot::Mutex;
use retry::{delay::Fixed, retry};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use sysfs_gpio::{Direction, Edge, Pin};

pub struct Tacho {
    inner: Arc<Mutex<InnerTacho>>,
}

struct InnerTacho {
    pub running: bool,
    pin: Pin,
    poller: Option<sysfs_gpio::PinPoller>,
    last_instant: Option<Instant>,
    current_rpm: Option<u64>,
}

impl InnerTacho {
    pub fn get_rpm(&self) -> Option<u64> {
        self.current_rpm
    }

    fn destroy(&mut self) -> Result<(), String> {
        match self.pin.unexport() {
            Err(err) => Err(format!("Could not unexport GPIO Pin. {}", err)),
            _ => Ok(()),
        }?;

        self.running = false;
        Ok(())
    }

    fn init(&mut self) -> Result<(), String> {
        match self.pin.export() {
            Err(err) => Err(format!("Could not export GPIO Pin. {}", err)),
            _ => Ok(()),
        }?;

        self.set_pin_direction()?;
        self.set_pin_edge()?;

        match self.pin.get_poller() {
            Ok(poller) => {
                self.poller = Some(poller);
                Ok(())
            }
            Err(err) => Err(format!("Could not create poller for GPIO. {}", err)),
        }?;

        self.running = true;
        Ok(())
    }

    fn set_pin_direction(&mut self) -> Result<(), String> {
        match retry(Fixed::from_millis(500), || {
            match self.pin.set_direction(Direction::In) {
                Err(err) => return Err(format!("Could not set direction on GPIO. {}", err)),
                Ok(_) => Ok(()),
            }
        }) {
            Err(retry::Error::Operation {
                error,
                total_delay: _,
                tries: _,
            }) => return Err(error),
            _ => Ok(()),
        }
    }

    fn set_pin_edge(&mut self) -> Result<(), String> {
        match retry(Fixed::from_millis(500), || {
            match self.pin.set_edge(Edge::RisingEdge) {
                Err(err) => return Err(format!("Could not set edge on GPIO. {}", err)),
                Ok(_) => Ok(()),
            }
        }) {
            Err(retry::Error::Operation {
                error,
                total_delay: _,
                tries: _,
            }) => return Err(error),
            _ => Ok(()),
        }
    }

    fn next_rpm_sample(&mut self) -> Result<(), String> {
        let poller = self.poller.as_mut().unwrap();
        match poller.poll(100) {
            Ok(None) => self.current_rpm = Some(0),
            Ok(_) => self.sample(),
            Err(err) => return Err(format!("Could not poll GPIO. {}", err)),
        };

        Ok(())
    }

    fn sample(&mut self) {
        match self.last_instant {
            Some(last_instant) => {
                let frequency = 1.0 / (Instant::now() - last_instant).as_secs_f64();
                self.current_rpm = Some(((frequency * 60.0) / 2.0) as u64);
            }
            None => {}
        }

        self.last_instant = Some(Instant::now());
    }
}

impl Tacho {
    pub fn new(pin_number: u64) -> Self {
        let pin = Pin::new(pin_number);

        let inner_tacho = InnerTacho {
            running: false,
            pin: pin,
            poller: None,
            last_instant: None,
            current_rpm: None,
        };

        Tacho {
            inner: Arc::new(Mutex::new(inner_tacho)),
        }
    }

    pub fn start(&mut self) {
        let inner = self.inner.clone();

        match inner.lock().init() {
            Err(err) => {
                error!(
                    "Tacho could not be initialized. Is your configuration correct? {}",
                    err
                )
            }
            Ok(_) => {}
        }

        thread::spawn(move || loop {
            let mut locked_inner = inner.lock();
            if locked_inner.running {
                match locked_inner.next_rpm_sample() {
                    Err(err) => warn!("Could not sample RPM due to error: {}", err),
                    Ok(_) => {}
                }
            } else {
                return;
            }
        });
    }

    pub fn stop(&mut self) {
        match self.inner.lock().destroy() {
            Err(err) => {
                warn!("Tacho could not be destroyed properly. {}", err)
            }
            Ok(_) => {}
        }
    }

    pub fn get_rpm(&self) -> Option<u64> {
        self.inner.lock().get_rpm()
    }
}

impl Drop for Tacho {
    fn drop(&mut self) {
        self.stop();
    }
}
