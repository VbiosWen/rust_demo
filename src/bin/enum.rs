use crate::IpAddrKind::IPV6;

enum IpAddrKind {
    IPV4,
    IPV6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}",self)
    }
}

fn route(ip_type: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::IPV4;
    let six = IpAddrKind::IPV6;

    let home = IpAddr {
        kind: IpAddrKind::IPV4,
        address: String::from("127.0.0.1"),
    };
    let loopBack = IpAddr {
        kind: IpAddrKind::IPV6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();
}