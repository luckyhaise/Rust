// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn tempconverter(value:i32,unit:String) -> String{
    match unit.as_str(){
    "f"| "F" => format!("{}°C",(value-32)*5/9),
    "C"| "c" => format!("{}°F",value*9/5+32),
    _ => "invalid Unit".to_string(),
    }
}















pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![tempconverter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
