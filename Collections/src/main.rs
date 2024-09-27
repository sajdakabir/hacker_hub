fn main() {
    // let mut vec =Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);

    // let vec=vec;

    let mut v1= vec![1,2,3,4];

    //get vetcor value

    // using index method
    // let value = &v1[2];

    // using get 

    // let value = v1.get(30).unwrap();
    // let value = v1.get(30).unwrap_or(&-1);
    // let value = match v1.get(20) {
    //     Some(val)=> val,
    //     None =>{
    //         println!("index out of bound");
    //         &-1
    //     },
    // };

    for i in &mut v1 {
        println!("the value is {i}");
        (*i) *= 2;
    }
    println!("tha value of vector is {:?} ", v1);



}
