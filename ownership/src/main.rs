fn main() {
    // let s1 = String::from("Hello");
    // let s2= s1.clone(); //it'v very expensive
    // let s2= s1;
    

    // println!("S= {s1}");

    let mut s = String::from("Hello");
    // take_ownership(s);
    // println!("S= {s}");

    // let s= give_ownership();
 takes_and_gives_ownership(&mut s);
}

// fn take_ownership (s: String){
//     println!("This is from take ownership {s}.")
// }

// fn give_ownership() -> String{
     
//     let s= String::from(" World");
//     s

// }

fn takes_and_gives_ownership(s: &mut String){
    let len= s.len();
    s.push_str("bye");
    println!("s ={s} and len is {len}");
}