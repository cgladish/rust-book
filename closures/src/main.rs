fn main() {
    let mut x = 0;
    let counter = |n: &mut i32| *n += 1;

    println!("{x}");

    counter(&mut x);

    println!("{x}");


    let mut fib = vec![1, 1];
    let mut push_next = || {
        let second_last = fib[fib.len() - 2];
        let last = fib[fib.len() - 1];
        fib.push(second_last + last);
    };

    for _ in 1..20 {
        push_next();
    }

    let fib_str = int_vec_to_string(&fib);
    println!("{fib_str}");
}

fn int_vec_to_string(v: &Vec<i32>) -> String {
    v
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}