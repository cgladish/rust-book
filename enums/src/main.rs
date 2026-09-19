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
}
