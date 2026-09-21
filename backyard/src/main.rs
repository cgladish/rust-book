use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let asparagus = Asparagus {
        length: 8
    };

    println!("Asparagus {asparagus:?}")
}