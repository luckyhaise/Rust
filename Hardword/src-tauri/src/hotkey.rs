use std::{thread,sync::{Arc, Mutex}, time::{Duration,Instant}};
use tauri_plugin_clipboard_manager::ClipboardExt;
use rdev::{listen, EventType,Event,Key};
use arboard::Clipboard;
#[tauri::command]

fn hotkey() {
  let last_pressed=Arc::new(Mutex::new(Instant::now() - Duration::from_secs(1)));
  let last_pressed_clone= Arc::clone(&last_pressed);
//   to track if control is pressed
  let control= Arc::new(Mutex::new(false));
  let control_clone= Arc::clone(&control);

  std::thread::spawn(move || { 
    if let Err(e) = listen(move|event:Event|{
        match event.event_type{
            EventType::KeyPress(key)=> {if (key == Key::ControlLeft) ||(key== Key::ControlRight){
                *control_clone.lock().unwrap() = true;
            }
            if key == Key::KeyC && *control_clone.lock().unwrap(){
                let now = Instant::now();
                let mut since =  last_pressed_clone.lock().unwrap();
                if now.duration_since(*since) < Duration::from_millis(400){
                    let clipboard=Clipboard::
                }
                *since= Instant::now();
            }
        }
             EventType::KeyRelease(key) => {if (key == Key::ControlLeft) ||(key== Key::ControlRight){
                *control_clone.lock().unwrap() = false;}

             }
             _ => ()
        }  
    }) {eprintln!("{:?}",e);}
    
  });

  loop {
        thread::sleep(Duration::from_secs(1));
    }

}