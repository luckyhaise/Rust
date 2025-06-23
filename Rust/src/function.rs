use std::i32;

pub fn temp(){
    for number in (0..12).rev()  {
        println!("{}",number)
    }
}
pub fn fun(x:i32,y:i32) -> i32 {
    return x*x +y*y +2*x*y;

}
pub fn main(){
    let s =String::from("Hello");
    for c in s.chars() {
        println!("{}",c);
    }
}pub fn list(){
    let list: Vec<i32> = [1,2,3,4,5,6,7,8,9].to_vec();
    let pist = list.iter();
    println!("{:?} ", pist.collect::<Vec<&i32>>());
    
    
}