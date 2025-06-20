use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is {}",secret_number);
    println!("Enter your guess :");
    loop {
        let mut guess =String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Could not read file");

        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a number");
                continue;

            }
            }; 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You have entered smaller number :"),
            Ordering::Greater =>println!("You have Entered Greater number :"),
            Ordering::Equal => {println!("You won :");
            break;
        }
        }




    }
    
}
    
