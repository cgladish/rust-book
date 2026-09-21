use crate::garden::vegetables::Asparagus;

pub mod garden;

use restaurant::{front_of_house::Breakfast, enter};
use restaurant::*;
use restaurant::{self, front_of_house::hosting};

fn main() {
    let asparagus = Asparagus {
        length: 8
    };

    println!("Asparagus {asparagus:?}");

    enter();
    front_of_house::hosting::add_to_waitlist();
}