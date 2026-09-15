const THREE_HOURS_IN_SECONDS: u32  = 3 * 60 * 60;

fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");

    println!("Three hours is {THREE_HOURS_IN_SECONDS}");

    let x = x + 1;
    println!("x is {x}");

    {
        let x = x * 2;
        println!("X in inner scope is {x}");
    }

    println!("x is {x}");

    let mut spaces = "   ";
    let spaces = 123;

    println!("spaces is {spaces}");
}
