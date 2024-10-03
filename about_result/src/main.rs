fn main() {
    let r =divition(4, 0).unwrap_or(-1);

    println!("r is {r}");
}

fn divition(x: i32, y: i32) -> Result<i32, String>{
    if y==0 {
      return  Err(String::from("can't divided by zero"));
    }
    Ok(x/y)
}