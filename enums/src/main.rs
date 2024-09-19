

// #[derive(Debug)]
// enum KindIp {
//     V4(String),
//     V6(String)
// }


// fn main() {
//     let x= KindIp::V4(String::from("hfd"));
//     get_ip_add(x);
// }

// fn get_ip_add(ip:KindIp){
//     println!("the ip add is {:?}", ip);
// }

enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChnaageColor(i32, i32,i32)
}
fn main() {
    let op = Option::Some(1);

    let x= 2;
    let sum= x+op.unwrap();
    let sum1= x+op.unwrap_or(0);

    let anyType= Option::Some(3.4);
}