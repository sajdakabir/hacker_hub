fn main() {
    // create new string 

//     let mut s= String::new();
//     s.push_str("saju");
//     println!("the value is  {s}");

//     // initial data 

//     let data ="i am a data";
//     let s1 =data.to_string();

//     // can use directly
//     let s2= "im a data".to_string();

//     let s3 = String::from("im some data");


let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s2 + &s1;
println!("the value s3 is {s3}");
// println!("the value s2 is {s2}");
println!("the value s1 is {s1}");
}
