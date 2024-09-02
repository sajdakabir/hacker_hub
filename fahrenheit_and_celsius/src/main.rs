use std::io;

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius!");

    println!("Enter a temperature in Celsius: ");

    let mut cel= String::new();
    io::stdin()
        .read_line(&mut cel)
        .expect("Failed to read value");


    // need to convert value
    let cel: f32= match cel.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return;
        },
    };

println!("Entered Celsius value is {cel}");


let fah = (cel * 9.0/5.0) + 32.0;

println!("Fahrenheit value is {fah}F");



}
