

fn main() {
   

let x= None;
println!("the value is {:?} ", plase_one(x));
println!("the value is {:?} ",sum(4, Some(4)));
println!("the value is {:?} ",sum(4, None));


}

fn plase_one(x:Option<i32>)-> Option<i32>{
    match x {
        Some(i)=>Some(i+1),
        None=>None
    }
}

fn sum (x: i32, y : Option<i32>)-> i32{
    match y {
        Some(i)=> x+i,
        None=> x
    }

}
