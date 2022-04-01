use crate::{search, search_case_insensitive};

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["Safe, fast, productive."], search(query, contents))
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
}

