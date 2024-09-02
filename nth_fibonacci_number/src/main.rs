use std::io;
fn main() {
    println!("nth Fibonacci number!");

    println!("Enter a number");

    let mut num= String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read value");

    let num:i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        }
    };

    println!("Entered number {num}");
    
   let ans = fibonacci( num);

   println!("Fibonacci number is {ans}");

}

fn fibonacci(num: i32) -> i32 {
    if num <=1 {
        return num;
    }

    fibonacci(num-1)+fibonacci(num-2)
}