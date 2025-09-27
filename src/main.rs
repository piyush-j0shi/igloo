fn longest_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let result = longest_string("thisislongeststring", "that");
    println!("result : {result}");
}
