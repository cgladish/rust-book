use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name} {team_score}");

    for (team_name, team_score) in &scores {
        println!("{team_name} {team_score}");
    }

    let mut flavors = HashMap::new();
    let raspberry = String::from("Raspberry");
    flavors.insert(String::from("Blue"), raspberry);

    // hashmap took ownership of raspberry
    // println!("Raspberry {raspberry}");

    for (color, flavor) in &flavors {
        println!("{color} {flavor}");
    }

    // Replace
    flavors.insert(String::from("Blue"), String::from("Blueberry"));
    for (color, flavor) in &flavors {
        println!("{color} {flavor}");
    }

    flavors.entry(String::from("Blue")).or_insert(String::from("Cotton Candy"));
    flavors.entry(String::from("Green")).or_insert(String::from("Apple"));
    for (color, flavor) in &flavors {
        println!("{color} {flavor}");
    }

    let text = "hey hey you you I don't like";
    let mut word_count: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    println!("{word_count:?}");
}
