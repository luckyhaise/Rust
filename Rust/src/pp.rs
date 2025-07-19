use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub  fn main() {
    let last_ctrl_c = Arc::new(Mutex::new(Instant::now() - Duration::from_secs(2)));
    let last_ctrl_c_clone = Arc::clone(&last_ctrl_c);

    let ctrl_down = Arc::new(Mutex::new(false));
    let ctrl_down_clone = Arc::clone(&ctrl_down);

    thread::spawn(move || {
        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    // Track when Ctrl is pressed
                    if key == Key::ControlLeft || key == Key::ControlRight {
                        *ctrl_down_clone.lock().unwrap() = true;
                    }
                    // When C is pressed while Ctrl is down
                    if key == Key::KeyC && *ctrl_down_clone.lock().unwrap() {
                        let now = Instant::now();
                        let mut last = last_ctrl_c_clone.lock().unwrap();
                        if now.duration_since(*last) < Duration::from_millis(500) {
                            println!("🚀 Double Ctrl+C detected!");
                        }
                        *last = now;
                    }
                }
                EventType::KeyRelease(key) => {
                    // Track when Ctrl is released
                    if key == Key::ControlLeft || key == Key::ControlRight {
                        *ctrl_down_clone.lock().unwrap() = false;
                    }
                }
                _ => {}
            }
            // No return value needed
        };

        if let Err(err) = listen(callback) {
            eprintln!("Error starting listener: {:?}", err);
        }
    });

    // Keep main alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
