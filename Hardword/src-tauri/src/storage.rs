use rusqlite::{params,Connection,Result};
use serde::Serialize;
use std::sync::{Mutex};
pub struct Dbcon(pub Mutex<Connection>);
#[derive(Serialize)]
struct Word {
    id: i32,
    word: String,
    meaning: String,
}
#[tauri::command]


fn insert(state: tauri::State<Dbcon> ,word:String,meaning:String)->Result<(), String>{
 let con  = state.0.lock().map_err(|e|e.to_string())?;
     con.execute(   
        "INSERT INTO words(word,meaning) VALUES(?,?)",
        params![word,meaning]
        ).map_err(|e|e.to_string())?;
   Ok(())
   } 





#[tauri::command]
fn get_words(state: tauri::State<Dbcon>)-> Result<Vec<Word>,String>{
   let con = state.0.lock().map_err(|e|e.to_string())?;
   let mut stmt=con.prepare("SELECT id ,word ,meaning FROM words").map_err(|e|e.to_string())?;
   let word_iter = stmt
   .query_map([],|row|{
      Ok(Word{
         id: row.get("id")?,
         word: row.get("word")?,
         meaning: row.get("meaning")?,
      })

   }).map_err(|e|e.to_string())?;
   let mut words = Vec::new();
   for word in word_iter{
      words.push(word.map_err(|e|e.to_string())?);
   }

Ok(words)
}









pub fn main() -> Result<(), Box<dyn std::error::Error>>{
   let con = Connection::open("mydatabase.db")?;
   

   con.execute("
    CREATE TABLE IF NOT EXISTS words(
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL,
    meaning TEXT NOT NULL
    
    )",
       [] )?;

       let db = Dbcon(Mutex::new(con));



   tauri::Builder::default()
   .manage(db)
   .invoke_handler(tauri::generate_handler![insert,get_words])
   .run(tauri::generate_context!())?;
  Ok(())
  }