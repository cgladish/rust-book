fn main() {
    let string1 = String::from("string1");
    {
        let string2 = "string2";
        let result = longest(string1.as_str(), string2);
        println!("{result}");
    }

    let string2 = "string2";
    let result;
    {
        result = longest(string1.as_str(), string2);
    }
    println!("{result}"); // This works because of lifetime annotations

    // The following does NOT work because string2 doesn't live long enough
    // let result;
    // {
    //     let string2 = String::from("string3");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("{result}"); // This works because of lifetime annotations

    let excerpt = Excerpt {
        text: "Lorem ipsum"
    };
    let text = excerpt.text;
    println!("{text}");
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

// str2 not used so don't need lifetime specifier on it
fn first<'a>(str1: &'a str, str2: &str) -> &'a str {
    str1
}

struct Excerpt<'a> {
    text: &'a str
}

// Special static lifetime
const s: &'static str = "static str";