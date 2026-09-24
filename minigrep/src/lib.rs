pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&str> = vec![];

  for line in contents.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
      result.push(line);
    }
  }

  result
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn one_result() {
      let query = "duct";

      let result = search(query, CONTENTS);
      assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn case_insensitive() {
      let query = "Duct";

      let result = search(query, CONTENTS);
      assert_eq!(vec!["safe, fast, productive."], result);
    }
}