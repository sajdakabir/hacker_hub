

fn main() {
   

let x= None;
println!("the value is {:?} ", plase_one(x));

}

fn plase_one(x:Option<i32>)-> Option<i32>{
    match x {
        Some(i)=>Some(i+1),
        None=>None
    }
}
