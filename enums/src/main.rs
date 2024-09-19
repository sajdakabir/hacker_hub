

#[derive(Debug)]
enum KindIp {
    V4,
    V6
}
struct IpAddress{
    kind: KindIp,
    add: String
}

impl IpAddress {
    fn new(add: &str)-> Self{
        Self { kind: KindIp::V4, add: add.to_string() }
    }
}

fn main() {
    let google =IpAddress::new("0.0.0.0");
    get_ip_add(google);
}

fn get_ip_add(ip:IpAddress){
    println!("the ip add is {} {:?}", ip.add, ip.kind);
}