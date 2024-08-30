fn main() {
    println!("!!loop loop!!");

    let mut num = 1;
    let x = loop {
       println!("The num is {num}");

       if num ==10 {
        break 50;
       }

       num = num+1;
    };

    println!("the value of x is {x}")
}
