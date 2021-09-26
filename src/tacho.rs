use parking_lot::Mutex;
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
    last_instant: Option<Instant>,
    current_rpm: Option<u64>,
}

impl InnerTacho {
    pub fn get_rpm(&self) -> Option<u64> {
        self.current_rpm
    }

    fn destroy(&mut self) {
        self.running = false;
        self.pin.unexport().unwrap();
    }

    fn init(&mut self) {
        self.running = true;
        self.pin.export().unwrap();
    }

    fn next_rpm_sample(&mut self) {
        // maybe one needs the closure with_exported?
        self.pin.set_direction(Direction::In).unwrap();
        self.pin.set_edge(Edge::RisingEdge).unwrap();

        let mut poller = self.pin.get_poller().unwrap();
        let result = poller.poll(100);

        match result {
            Ok(None) => self.current_rpm = Some(0),
            Ok(_) => self.sample(),
            Err(_) => {}
        };
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
            last_instant: None,
            current_rpm: None,
        };

        let tacho = Tacho {
            inner: Arc::new(Mutex::new(inner_tacho)),
        };

        tacho
    }

    pub fn start(&mut self) {
        let inner = self.inner.clone();

        thread::spawn(move || {
            inner.lock().init();
            loop {
                let mut locked_inner = inner.lock();
                if locked_inner.running {
                    locked_inner.next_rpm_sample()
                } else {
                    return;
                }
            }
        });
    }

    pub fn stop(&mut self) {
        self.inner.lock().destroy();
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
