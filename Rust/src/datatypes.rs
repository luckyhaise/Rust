use std::io;
pub fn tuples(){
let report: (&str,&str,i32)=("Lucky","Maths",760);
let (name,subject,score)=report;
println!("My name is{} and i recieved {} in {}",name,score,subject);
println!("what is your Name");
println!("how many students do you have of maths");

let student_count= loop {
    let mut  count =String::new();
   io::stdin()
   .read_line(&mut count)
   .expect("Please enter a valid Number");

   match count.trim().parse::<i32>() {
    Ok(num) => break num,
    Err(_) => println!("Please enter a vaild integer"),
       
   }
};









  struct StudentData {
    name: String,
    score: i32,
     subject: String,


 }
let mut data:Vec<StudentData>=Vec::new();



for i  in 0..student_count {
 
 println!("Enter the details of {} student",i);
   
 let mut name =String::new();
 let mut score_input =String::new();
 let mut subject =String::new();

 println!("Enter your name");
 io::stdin()
 .read_line(&mut name)
 .expect("Please enter a accurate value");
 
 println!("Enter your score");
 io::stdin()
 .read_line(&mut score_input)
 .expect("Please enter a accurate value");
 
 println!("Enter your subject");
 io::stdin()
 .read_line(&mut subject)
 .expect("Please enter a accurate value");

let score:i32= score_input.trim().parse().expect("Please Enter a Number");
 
 let student = StudentData{
    name:name.trim().to_string(),
    score,
    subject:subject.trim().to_string(),
 };
    data.push(student);
}

for student in &data
{
    println!("{}Got {} marks in {}",student.name, student.score,student.subject)
}

    
}


