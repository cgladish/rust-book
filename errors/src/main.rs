use std::fs::File;
use std::io::{Error, ErrorKind, Write};

fn main() {
    // Using match
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create("hello.txt") {
                        Ok(f) => {
                            println!("Created file hello.txt");
                            f
                        },
                        Err(error) => {
                            panic!("Failed to create file hello.txt {error:?}");
                        }
                    }
                }
                _ => {
                    panic!("Failed to open file hello.txt {error:?}");
                }
            }
        }
    };

    // Using closures
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = greeting_file_result.unwrap_or_else(|error| {
        if (error.kind() != ErrorKind::NotFound) {
            panic!("Failed to open file hello.txt {error:?}");
        }
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Failed to create file hello.txt {error:?}");
        })
    });

    // Straight up panic if no file
    let greeting_file_result = File::open("hello.txt")
        .expect("Unable to open file hello.txt");

    let greeting_file = open_or_create_file_with_content("hello.txt", "hihi").unwrap();
}

fn open_or_create_file_with_content(path: &str, content: &str) -> Result<File, Error> {
    let mut file = File::open(path).or_else(|error| {
        match error.kind() {
            ErrorKind::NotFound => File::create("hello.txt"),
            _ => Err(error)
        }
    })?;

    file.write_all(content.as_bytes())?;
    Ok(file)
}