

#[derive(Debug)]
enum KindIp {
    V4,
    V6
}
struct IpAddress{
    kind: KindIp,
    add: String
}

fn main() {
    let google =IpAddress{
        kind:KindIp::V4,
        add: String::from("0.0.0.0")

    };
    get_ip_add(google);
}

fn get_ip_add(ip:IpAddress){
    println!("the ip add is {} {:?}", ip.add, ip.kind);
}