#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)] // so we can inspect the state in a minute
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl IpAddr {
    fn get_address(&self) -> &str {
        match self {
            IpAddr::V4(addr) | IpAddr::V6(addr) => addr,
        }
    }
}

fn route(ip_type: &IpAddrKind) {
    println!("{:?}", ip_type);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(&four);
    route(&six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home address: {}", home.get_address());
    println!("Loopback address: {}", loopback.get_address());

    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(coin));
}
