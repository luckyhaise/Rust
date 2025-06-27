use rusqlite::{params,Connection,Result};
use tauri::generate_context;
use std::sync::{Arc,Mutex};
pub struct Dbcon(pub Mutex<Connection>);
#[tauri::command]


fn insert(con: Connection,words:String,meaning:String)->Result<()>{
 
     con.execute(   
        "INSERT INTO words(word,meaning) VALUES(?,?)",
        (words,meaning)
        )?;
       let q = con.execute( 
        "SELECT * FROM words;"
      ,  [])?;
      println!("{}",q);
      Ok(())
   }
        






  pub fn main(){
   let con = Connection::open("mydatabase.db")?;
   con.execute("
    CREATE TABLE IF NOT EXISTS words(
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL,
    meaning TEXT NOT NULL
    
    )",
       [] );
   tauri::Builder::default()
   .plugin(tauri_plugin_opener::init())
   .invoke_handler(tauri::generate_handler![insert])
   .run(generate_context!())
   .expect("Unable to run the app");
  }