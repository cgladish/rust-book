fn main() {
    let guess:u32 = "42".parse().expect("Not a number!");

    println!("guess is {guess}");

    let too_big_result = std::panic::catch_unwind(|| {
        return 2_u8.pow(9);
    });

    match too_big_result {
        Ok(value) => {
            println!("should not reach this {value}");
        },
        Err(_) => {
            println!("value too big. this is good");
        }
    }

    let char = '😻';
    let mut str_appended = format!("Hi {char}");
    str_appended.push(char);
    println!("appended string: {str_appended}");

    let tup:(&str, i32, f64)  = ("123", 123, 123.3);
    let (x, _, _) = tup;
    let y = tup.1;
    let z = tup.2;
    println!("x {x}, y {y}, z {z}");

    let mut arr: [i32; 3] = [1, 2, 3];
    arr = [4, 5, 6];

    let a = arr[0];

    println!("arr {a}");

    let arr2 = [5; 20];
}
