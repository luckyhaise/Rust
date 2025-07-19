use std::sync::{Arc,Mutex};
use std::time::{Instant,Duration};
use tauri_plugin_global_shortcut::{Shortcut,Modifiers,Code,ShortcutState,GlobalShortcutExt};
use tauri::{Emitter};
use tauri_plugin_clipboard_manager::{ClipboardExt};

pub fn run() {
	tauri::Builder::default()
    .plugin(tauri_plugin_global_shortcut::Builder::new().with_handler({
        let last_time = Arc::new(Mutex::new(Instant::now() - Duration::from_secs(1)));
        let last_time_clone = Arc::clone(&last_time);
        move|app,shortcut,event|{
            if shortcut==  &Shortcut::new(Some(Modifiers::CONTROL), Code::KeyC){
                if event.state() == ShortcutState::Pressed{
                    let now= Instant::now();
                    let mut last = last_time_clone.lock().expect("error in code");

                   if now.duration_since(*last) < Duration::from_millis(400){             
                   if let Ok(content) = app.clipboard().read_text(){
                    app.emit("hardword_content" , &content).unwrap();
                    println!("{}",&content)
                    
                   }
                   }
                   *last= Instant::now();

                }
                 
            }
        
        }

    
    }).build())
    .plugin(tauri_plugin_clipboard_manager::init())
    
    .setup(|app| {
        let shortcut = Shortcut::new(Some(Modifiers::CONTROL ), Code::KeyC);
        app.global_shortcut().register(shortcut)?;
        Ok(())
    }
    )
    .run(tauri::generate_context!())
    .expect("Error running hotkey")
}