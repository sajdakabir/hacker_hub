use std::ffi::c_long;

fn main() {
  // Rust is a statically typed language


  // let mut x =5;
  // println!("The Value of x is {}", x);
//   in rust variable is by deafult is immutable 
// canst can't be mutable


// x= 25;

// println! ("The value of x is {}", x);

// // constant
// // Constants aren’t just immutable by default—they’re always immutable. 
// const PI: u32= 3;
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


// shadowing

let x=5;
let x= x+1;
// we’re effectively creating a new variable when we use the let keyword again
{
  let x= x+2;
  println!("The value of x in inner scope is {}",x); //8
}
println!("The value of x is {}",x); 
// we can't use mute on shadowing

// unit datatype 
let y :()=();
println!(" this is {:?}",y);


// floting

let a:f32=3.5;
let b:u8= a as u8- 1;
println!("{}",a);
println!("{}",b);
}
