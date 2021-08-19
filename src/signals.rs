use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct SignalHandler {
    terminate_flag: Arc<AtomicBool>,
}

impl SignalHandler {
    pub fn new() -> Option<SignalHandler> {
        let terminate_flag = Arc::new(AtomicBool::new(false));

        for sig in TERM_SIGNALS {
            match flag::register(*sig, Arc::clone(&terminate_flag)) {
                Err(_) => return None,
                _ => {}
            }
        }

        Some(SignalHandler {
            terminate_flag: terminate_flag,
        })
    }

    pub fn should_terminate(&self) -> bool {
        self.terminate_flag.load(Ordering::Relaxed)
    }
}
