enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String),
}
fn main() {
    let home = IpAddress::V4(127,0,0,1);
    let loopback = IpAddress::V6(String::from("::1"));
}
