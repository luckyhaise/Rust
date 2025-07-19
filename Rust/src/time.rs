use std::time::{Duration,Instant};
pub fn time()
{let x = Instant::now();
println!("{:?}",x)
}