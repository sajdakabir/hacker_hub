fn main() {
   let mut s = String::from("Hello World");
   let res= first_word(&s);

   println!("the value is {}", res.len());
   s.clear();
}
fn first_word(s: &String)-> &str{
   let bytes= s.as_bytes();
   for (i, &item) in bytes.iter().enumerate(){
      if item == b' ' {
         return &s[..i];
      }
   }
  &s[..]
}
