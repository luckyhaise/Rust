
// Translator
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Sense{
    glosses: Option<Vec<String>>,
    examples: Option<Vec<String>>,

}

#[derive(Debug, Deserialize)]
struct Entry{
    word:String,
    lang: String,
    senses: Option<Vec<Sense>> 
}


//Calling API, Using wikitionary

pub async fn translate()->Result<Vec<String>, String>{
  let word =String::from("love");

let url=format!("https://kaikki.org/dictionary/English/meaning/{}.json",word);
let response = reqwest::get(&url)
  .await
  .map_err(|e|format!("Request error{}",e))?;

  if !response.status().is_success(){
       return Err(format!("Failed with status: {}", response.status()));
  }


   let  entries:Vec<Entry>=response
   .json()
   .await
   .map_err(|e|e.to_string())?; 
 let mut glosses=Vec::new();
   for entry in entries{
    if let Some(senses) = entry.senses{
      for sense in senses{
        if let Some(gs) = sense.glosses{
          glosses.extend(gs)
        }

      }
    }
    
   }

Ok(glosses)

}





 