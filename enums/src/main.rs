

#[derive(Debug)]
enum KindIp {
    V4(String),
    V6(String)
}


fn main() {
    let x= KindIp::V4(String::from("hfd"));
    get_ip_add(x);
}

fn get_ip_add(ip:KindIp){
    println!("the ip add is {:?}", ip);
}