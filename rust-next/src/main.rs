fn main() {
   let x = 10;
   print!("x: {}\n",x );

//    string

let _greeting = String::from("hello world");
// println!("{}", greeting);


let _name = String::from("Sajda");
// println!("{}", name);

// conditionals

let is_even= false;

if is_even {
    println!("I am even")
} else{
    println!("I an odd")
} 
// another example of condition 
let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


// loops   --> rust has 3 types of loops -->> loop, while and for



for i in 0..10 {
    println!("hey {}", i)
}
    
}
