use std::fmt::Display;

fn longest_with_announcement<'a, T: Display>(str1: &'a str, str2: &'a str, ann: T) -> &'a str {
    println!("ann is : {ann}");
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let longest = longest_with_announcement(
        "this is an announcement with longest",
        "short announcement",
        3,
    );
    println!("longest is : {longest}");
}
