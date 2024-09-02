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

    println!("the value of x is {x}");


    // for loop

    let arr: [i32; 4] =[1,2,34,44];

    for x in arr {
        println!("x is {x}");
    }

    for i in (1..4).rev() {
        println!("x is {i}");
    }

}
