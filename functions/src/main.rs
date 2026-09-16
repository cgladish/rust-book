fn main() {
    let unit = "cm";
    another_function(1234, unit);

    let y = {
        let x = 1;
        x + 3
    };
    println!("y = {y}");

    let five_result = five();
    println!("5 = {five_result}");

    let six = plus_one(5);
    println!("6 = {six}");
}

fn another_function(x: i32, unit: &str) {
    println!("x is {x} {unit}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}