struct Ipv4Address {
    address: (u8, u8, u8, u8)
}

struct Ipv6Address {
    address: String
}

enum IpAddressKind {
    V4(Ipv4Address),
    V6(Ipv6Address),
    Other(String)
}

impl IpAddressKind {
    fn get_address(&self) -> String {
        match self {
            IpAddressKind::V4(ip) => {
                let (a, b, c, d) = ip.address;
                format!("{a}.{b}.{c}.{d}")
            }
            IpAddressKind::V6(ip) => ip.address.clone(),
            IpAddressKind::Other(ip) => ip.clone()
        }
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(num) => Some(num + 1),
        None => None
    }
}

enum SomeValue {
    IpAddressKind(IpAddressKind),
    String(String),
    Num(i32)
}

fn main() {
    let ipv4 = IpAddressKind::V4(Ipv4Address {
        address: (1, 2, 3, 4)
    });
    let ipv4_address = ipv4.get_address();

    let ipv6 = IpAddressKind::V6(Ipv6Address {
        address: String::from("::1")
    });
    let ipv6_address = ipv6.get_address();

    let other = IpAddressKind::Other(String::from("test"));
    let other_address = other.get_address();

    println!("V4 address {ipv4_address}");
    println!("V6 address {ipv6_address}");
    println!("Other address {other_address}");

    let maybe_num = Some(5);
    if let Some(value) = maybe_num {
        println!("Num is {value}");
    } else {
        println!("Num has no value");
    }

    match maybe_num {
        Some(value) => {
            println!("Num is {value}");
        }
        None => {
            println!("Num has no value");
        }
    }

    let maybe_num_plus1 = plus_one(maybe_num);
    let none = plus_one(None);

    let some_value = SomeValue::Num(3);
    match some_value {
        SomeValue::IpAddressKind(_) => {
            println!("Ip Address found")
        },
        _ => {
            println!("Other value found");
        }
    }

    if let SomeValue::IpAddressKind(_) = some_value {
        println!("Ip Address found");
        return;
    }

    let some_num = if let SomeValue::Num(num) = some_value {
        num
    } else {
        println!("Found non num value");
        return;
    };

    println!("Some num {some_num}");

    let SomeValue::Num(some_num) = some_value else {
        return;
    };

    println!("Some num {some_num}");
}
