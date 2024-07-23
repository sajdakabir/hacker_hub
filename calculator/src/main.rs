use std::io;

fn main() {
    println!("welcome:");
    let mut x =String :: new();
    let mut y= String::new();


    println!("Please enter a value for x:");
    io::stdin()
    .read_line(&mut x)
    .expect("Failed to read line");

    // let x: i32 = match x.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    let x: i32 = x.trim().parse().expect("Please type a number!");

    println!("Please enter a value for y:");
    io::stdin().read_line(&mut y).expect("Failed to read line");
    let y: i32 = y.trim().parse().expect("Please type a number!");

    println! ("sum:1\n sub:2\n mult:3\n div:4");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice:i32 = choice.trim().parse().expect("please type a number");


    match choice {
        1 => println!{"Result: {}",sum(x,y)},
        2 => println!{"Result: {}",sub(x,y)},
        3 => println!{"Result: {}",mul(x,y)},
        4 => {
            if y != 0 {
                println!("Result: {}", div(x, y));
            } else {
                println!("Error: Division by zero");
            }
        },
        _ => println!("Invalid choice"),

    }

}
fn sum (x:i32,y:i32) ->i32 {
    return x+y;
}
fn sub(x:i32, y:i32) ->i32 {
    return x-y;
}
fn mul (x:i32,y:i32) ->i32 {
    return x*y;
}
fn div(x:i32, y:i32) ->i32 {
    return x/y;
}
