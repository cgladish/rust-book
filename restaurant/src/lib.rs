pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            self::seat_at_table();
            super::serving::take_order();
        }

        fn seat_at_table() {}
    }

    mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    pub struct Breakfast {

    }

    pub enum Meat {
        Sausage(u8),
        Bacon(u8)
    }


}

use crate::front_of_house::hosting::add_to_waitlist as add;

pub use front_of_house::Meat; // Re-exports

pub fn enter() {
    add();
}

#[cfg(test)]
mod tests {
    use super::*;
    use front_of_house::Meat;

    #[test]
    fn it_works() {
        let result = crate::front_of_house::hosting::add_to_waitlist();
        let result = front_of_house::hosting::add_to_waitlist();

        let a = Meat::Sausage(3);
    }
}
