fn main() {
    // let mut vec =Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);

    // let vec=vec;

    let v1= vec![1,2,3,4];

    //get vetcor value

    // using index method
    // let value = &v1[2];

    // using get 

    // let value = v1.get(30).unwrap();
    // let value = v1.get(30).unwrap_or(&-1);
    let value = match v1.get(20) {
        Some(val)=> val,
        None =>{
            println!("index out of bound");
            &-1
        },
    };

    println!("tha value of vector is {:?} and the value is {value}", v1);



}
