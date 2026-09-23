use adder::add;

use crate::common::gen_args;

mod common;

// Contrived example to demonstrate usage of common folder
#[test]
fn add_works() {
    let (a, b) = gen_args();
    let result = add(a.into(), b.into());
    assert_eq!(result, (a + b).into());
}