enum IpAddressKind {
    V4,
    V6,
}
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}
fn main() {
    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
}
