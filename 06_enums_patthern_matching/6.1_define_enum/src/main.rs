#![allow(unused)]
fn main() {
    println!("Hello, world!");
    let ip: IpAddr = IpAddr::V4(127, 0, 0, 0);
}

enum IpAddr {
    // enum variant can have associated value
    V4(u8, u8, u8, u8),
    V6(String),
}

// implement for enum IpAddr
impl IpAddr {
    // --snip--
}
