fn main() {
    let mut s = "foo".to_string();

    let bar = "bar";

    s.push_str(bar);

    println!("bar is {bar}");
    println!("s is {s}");

    let foo = String::from("foo");
    let bar = String::from("bar");
    let foo_bar = foo + &bar;

    // Takes ownership
    // println!("foo is {foo}");
    println!("bar is {bar}");
    println!("foo_bar is {foo_bar}");


    let foo = String::from("foo");
    let bar = String::from("bar");
    let foo_bar = format!("{foo}{bar}");

    // Doesn't take ownership
    println!("foo is {foo}");
    println!("bar is {bar}");
    println!("foo_bar is {foo_bar}");

    let hello = "дh123";
    let д = &hello[..2]; // д actually takes 2 bytes
    println!("д is {д}");

    println!("chars");
    for c in hello.chars() {
        println!("{c}");
    }

    println!("bytes");
    for b in hello.bytes() {
        println!("{b}");
    }
}
