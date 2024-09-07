fn main() {

    let mut s = String::from("Hello");
    let r1= &s;
    let r2= &s;
    println!("r1= {r1} , r2 ={r2},");
    let r3= &mut s;
    // println!("r1= {r1} , r2 ={r2},r3={r3}");
    println!("r3={r3}");

    let test = dangle();

}

fn dangle ()-> &String{
    let s = String::from("hey");
    &s
}
