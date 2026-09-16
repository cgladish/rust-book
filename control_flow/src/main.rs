fn main() {
    let number = if true { 3 } else { 5 };
    println!("number is {number}");

    let mut counter = 0;
    let result = 'counting_up: loop {
        counter += 1;

        if counter >= 10 {
            loop {
                counter -= 1;

                if counter < 0 {
                    break 'counting_up counter;
                }
            }
        }
    };
    println!("result is {result}");

    let mut counter = 0;
    while counter < 10 {
        counter += 1;
    }
    println!("result is {counter}");

    let a = [1, 1, 2, 3, 5, 8];
    for elem in a {
        println!("Fib {elem}");
    }

    for num in (1..4).rev() {
        println!("num {num}");
    }
}
