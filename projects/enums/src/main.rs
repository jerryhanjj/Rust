#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {
    println!("ip_type: {:?}", ip_type);
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}, six: {:?}", four, six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
