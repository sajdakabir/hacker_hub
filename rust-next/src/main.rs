// fn main() {
//    let _x = 10;
// //    print!("x: {}\n",x );

// //    string

// let _greeting = String::from("hello world");
// // println!("{}", greeting);


// let _name = String::from("Sajda");
// // println!("{}", name);

// // conditionals

// let is_even= false;

// if is_even {
//     println!("I am even")
// } else{
//     println!("I an odd")
// } 
// // another example of condition 
// let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");


// // loops   --> rust has 3 types of loops -->> loop, while and for



// for i in 0..10 {
//     println!("hey {}", i)
// }

// // example of loop

// loop {
//     println!("hey girl")    //--> need to stop it manuallly
// }

// // another exmaple

// let mut counter = 0;

//     let result = loop {
//         counter += 1;
//         println!("there am i {}", counter);

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");



// let mut x= 0;
// let ans= loop {
//     x+=1;
//     if x==5 {
//         break x;
//     }
// };

// println!("The ans is {ans}");


// // while loop

//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");


//     // loop in rev

//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");

// // Functuons

// hey_i_am_function();

// println!("i am the five function {}", five());

// println!("I am the sum {}", sum(3,5));
    
// }

fn hey_i_am_function() {
    println!("hello")
}

// rutern type funtion

fn five() -> i32{
    5
}

// now do some some

fn sum(x:i32, y:i32) -> i32 {
    x+y
}

// memory management
fn testing(){
    //declear a string
    let s1= String::from("hello world");
}




fn main(){
    let mut s1= String::from("hey ");
    update_word(&mut s1);
    println!("{}",s1)
}
fn update_word(s:&mut String){
    s.push_str("there")
}