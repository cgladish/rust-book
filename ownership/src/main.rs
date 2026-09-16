fn main() {
    let original = String::from("Hello");
    let moved = original;

    println!("{moved}, world!");

    let cloned = moved.clone();
    println!("{cloned}, world!");

    let kidnapped = String::from("I'm being kidnapped");
    take_ownership(kidnapped);

    let still_here = String::from("I'm still here!");
    takes_reference(&still_here);
    println!("{still_here}");

    let borrowed = String::from("I was borrowed!");
    let borrowed = borrows(borrowed);
    println!("{borrowed}");

    let created = gives_ownership();
    println!("I have been given a new baby string! {created}");

    let mut still_here_but_modified = String::from("I'm still here!");
    takes_reference_and_modifies(&mut still_here_but_modified);
    println!("{still_here_but_modified}");

    let mut s = String::from("I'm gonna get referenced!");
    {
        let r1 = &mut s;
        println!("First reference! {r1}");
    }
    let r2 = &mut s;
    println!("Second reference! {r2}");

    let r1 = &s;
    println!("First reference! {r1}");
    let r2 = &mut s;
    println!("Second reference! {r2}");


    // ---- Slices ----
    let s: String = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{hello} {world}");
    
    let hello = first_word(&s);
    println!("{hello} {world}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    assert_eq!(slice, [1, 2, 3]);
}

fn take_ownership(input: String) {
    println!("My string now! {input}");
}

fn takes_reference(input: &String) {
    println!("I took a reference to the input! {input}");
}

fn borrows(input: String) -> String {
    println!("Just taking for a little bit! {input}");

    input
}

fn gives_ownership() -> String {
    let output = String::from("I'm a new baby string!");
    println!("Created a new baby string! {output}");

    output
}

fn takes_reference_and_modifies(input: &mut String) {
    input.push_str(" A little extra...");
    println!("I took a reference to the input and added something! {input}");
}

fn first_word(s: &str) -> &str {
    for (i, char) in s.char_indices() {
        if char == ' ' {
           return &s[..i]; 
        }
    }

    return &s[..];
}