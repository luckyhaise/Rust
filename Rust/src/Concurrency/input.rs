use std::io;

pub fn main(){
    println!("Do you wish to read your data or store into it (Read/Store)");
   let input =  loop{
      let mut int =String::new();
    io::stdin()
    .read_line( &mut int)
    .expect("failed to read  the line");
      
      let int= int.trim()
      .to_uppercase();

    match int.as_str() {
        "READ" | "STORE" => { break int;
        } 

        _ =>{
         println!("Please Enter (Read or Store)");
         continue;
        }
    };




   };


          match input.as_str(){
      "READ" => {
         println!("You have chose to read");
         
       read();
   
      }
            
      "STORE" =>{println!("You have chose to store");
      
       store();
      
      }
      _ => unreachable!()

       };      
         
      


    }
   

   
   



fn read(){
   println!("you chose read function");

}
fn store(){
   println!("you chose store function");
}
