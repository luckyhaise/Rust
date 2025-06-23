use std::io;

pub fn FtoC(){
    println!("Do you want to convert to F or C(Enter F or C)");
    let ask =loop{
    let mut qsk = String::new();
    io::stdin()
    .read_line(&mut qsk)
    .expect("Unable to read line");
    let qsk = qsk.trim().to_uppercase();
    if qsk == "F" || qsk == "C" {
        break qsk;
    }
    else {
        println!("Please enter only 'F' or 'C'");
    }

    };

    println!("Please enter your value");


    let value:i32 = loop {
    let mut input = String::new();
       io::stdin()
       .read_line(&mut input)
       .expect("Please enter a number");
       
       match input.trim().parse::<i32>() {
       Ok(num) => break num,
       Err(_) => {
          println!("Please enter a number!");
          continue;
       }   
       
       }
    };
   
   if ask == "F"{
    let F: i32 = (value * 9/5)+ 32;
    println!("Ferinhiet is {}°F", F )
   }
   else {
       let C:i32 = (value - 32)*5/9;
       println!("Celsius is: {}°C",C)
   }
    

}