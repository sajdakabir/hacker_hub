fn main() {
    // let s1 = String::from("Hello");
    // let s2= s1.clone(); //it'v very expensive
    // let s2= s1;
    

    // println!("S= {s1}");

    let s = String::from("Hello");
    // take_ownership(s);
    // println!("S= {s}");

    // let s= give_ownership();
 takes_and_gives_ownership(&s);
}

// fn take_ownership (s: String){
//     println!("This is from take ownership {s}.")
// }

// fn give_ownership() -> String{
     
//     let s= String::from(" World");
//     s

// }

fn takes_and_gives_ownership(s: &String){
    let len= s.len();
    println!("s ={s} and len is {len}");
}