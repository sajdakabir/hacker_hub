fn main() {
    println!("Hello, world!");
    this_is_function(10);

    //expressions

    let y = {
        let x=10;
        x+1
    };
    println!("The value of y is: {y}");

    let sum = add_two_nums(1,1);
    println!("The value of sum is: {sum}");

    let res=is_even(2);
    println!("The value of res is: {res}");

    // if you are using if as a expression so if needed else 

    let x = if res {10} else {20};
    println!("The value of x is: {x}");
}


fn this_is_function(x: i32) {
    println!("I am a function:  {}", x);
}

//function with return values

fn add_two_nums(x:i32, y:i32)-> i32{
    x+y
}
// early return

fn is_even(x:i32) -> bool{
    if x%2 ==0 {
        return true;
    }
    false
}