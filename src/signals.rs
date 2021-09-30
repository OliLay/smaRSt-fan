use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::Error;

pub struct SignalHandler {
    terminate_flag: Arc<AtomicBool>,
}

impl SignalHandler {
    pub fn new() -> Result<SignalHandler, Error> {
        let terminate_flag = Arc::new(AtomicBool::new(false));

        for sig in TERM_SIGNALS {
            flag::register(*sig, Arc::clone(&terminate_flag))?;
        }

        Ok(SignalHandler {
            terminate_flag: terminate_flag,
        })
    }

    pub fn should_terminate(&self) -> bool {
        self.terminate_flag.load(Ordering::Relaxed)
    }
}
