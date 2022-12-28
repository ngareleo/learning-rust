#[derive(Debug)]
enum MemberType {
    Regular, 
    Admin
}

#[derive(Debug)]
enum IPAddressType {
    IPV4(String),
    IPV6(String)
}

impl IPAddressType {
    fn address(&self) {
        // some arbitrary code
    }
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    Arizona,
}

enum Coin {
    Penny,
    Nickle,
    Dime, 
    Quater(USState) // you can bind any dt on a enum
}

fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("From the state of {:?}", state);
            25
        }
    }
}

fn main() {
    let member_type = MemberType::Regular;
    let admin_user = MemberType::Admin;
    dbg!("{}", admin_user);
    dbg!("{}", member_type);

    let home_address = IPAddressType::IPV4(String::from("127.0.0.1"));
    let loopback_address = IPAddressType::IPV6(String::from("0.0.0.1"));

    dbg!("{}", home_address);
    dbg!("{}", loopback_address);

    println!("{}", value_in_cents(Coin::Quater(USState::Alaska)));

    let config_max = Some(232);
    let config_max_b: Option<String> = None; // Rust doesn't have a null type
    // This is how to declare a null

    // match statements are exhaustive. You need to account for all options
    match config_max {
        Some(max) => {
            println!("Max is {}", max);
        },
        _ => () // catch all
    };

    // to pick one option, use the if let
    if let Some(max) = config_max {
        println!("Max is {}", max);
    }else {
        println!("Max has not been declared");
    }

    if let Some(max) = config_max_b {
        println!("Max is {}", max);
    }else {
        println!("Max has not been declared");
    }

}
