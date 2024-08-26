fn main() {
  println!("hey there");
  // mutation
  let mut age = 10;

  age =10+1;
  println!("age is {age}");
  // shadowing 

  let age = age;

  println!("age is now {age}");
  const PI: f32 = 3.14;

  println!("this is a const {}", PI);
}
