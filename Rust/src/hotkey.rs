
use std::{thread,sync::{Arc, Mutex}, time::{Duration,Instant}};
use tauri_plugin_clipboard_manager::ClipboardExt;
use rdev::{listen, EventType,Key};



fn hotkey() {
  let last_pressed=Arc::new(Mutex::new(Instant::now() - Duration::from_secs(1)));
  let last_pressed_clone= Arc::clone(&last_pressed);
//   to track if control is pressed
  let control= Arc::new(Mutex::new(false));
  let control_clone= Arc::clone(&control);
 

  thread::spawn(move||{
    if let Err(e) = listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            if key == Key::ControlLeft || key == Key::ControlRight {
              *control_clone.lock().unwrap()= true;
                
            }
            if key == Key::KeyC && *control_clone.lock().unwrap() {
                let now= Instant::now();
                let last =last_pressed_clone.lock().unwrap();
                now.duration_since(*last) < Duration::from_millis(400); 
            }

        }
    }) {
        eprintln!("Error listening to keys: {:?}", e);
    }
  });
}