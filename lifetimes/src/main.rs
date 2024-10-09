fn main() {
//    let r;


// {
//     let x=3;
//     r= &x;
// }

//    println!("the value of r is {r}");

let s1= "sajda".to_string();
let s2= "Sajda Kabir".to_string();

let res= get_logest_string(&s1, &s2);
println!("res={res}");
}

fn get_logest_string<'a>(s1: &'a String, s2: &'a String)-> &'a String {
    if s1.len()>s2.len(){
        return s1;
    }
    s2
}