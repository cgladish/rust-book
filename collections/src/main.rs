use std::fmt;

enum PrimitiveValue {
    String(String),
    Float(f64),
    Int(i32)
}

impl fmt::Display for PrimitiveValue {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveValue::String(v) => write!(f, "{v}"),
            PrimitiveValue::Int(v) => write!(f, "{v}"),
            PrimitiveValue::Float(v) => write!(f, "{v}"),
        }
   } 
}

fn main() {
    let mut vec: Vec<i32> = vec![];

    vec.push(5);
    vec.push(6);
    vec.push(7);

    let third = &vec[2];
    let third = vec.get(2);

    match third {
        Some(v) => println!("third is {v}"),
        None => println!("third has no value")
    };

    for v in &vec {
        println!("Value is {v}");
    }

    for v in &mut vec {
        *v *= 2;
    }

    for v in &vec {
        println!("Value is {v}");
    }

    let mut vec: Vec<PrimitiveValue> = vec![
        PrimitiveValue::String(String::from("Heihei")),
        PrimitiveValue::Int(3)
    ];

    for v in &vec {
        println!("Value is {v}");
    }
}
