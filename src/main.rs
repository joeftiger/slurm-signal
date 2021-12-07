use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let cancel = Arc::new(AtomicBool::new(false));
    let _ = signal_hook::flag::register(signal_hook::consts::SIGINT, cancel.clone()).unwrap();

    println!("waiting for SIGINT...");
    loop {
        if cancel.load(Ordering::Relaxed) {
            println!("SIGINT trapped!");
            break;
        }

        thread::sleep(Duration::from_millis(250));
    }
}
