fn main() {
   let _x = 10;
//    print!("x: {}\n",x );

//    string

let _greeting = String::from("hello world");
// println!("{}", greeting);


let _name = String::from("Sajda");
// println!("{}", name);

conditionals

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

// example of loop

loop {
    println!("hey girl")    //--> need to stop it manuallly
}

// another exmaple

let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("there am i {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");



let mut x= 0;
let ans= loop {
    x+=1;
    if x==5 {
        break x;
    }
};

println!("The ans is {ans}");

    
}
