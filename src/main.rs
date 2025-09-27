#[derive(Debug)]
struct Excerpt<'a> {
    data: &'a str,
}

impl<'a> Excerpt<'a> {
    fn return_first_word(&self) -> &'a str {
        let words: Vec<&str> = self.data.split_whitespace().collect();
        words[0]
    }
}

fn main() {
    let our_struct = Excerpt {
        data: "this is a string",
    };
    let first_word = our_struct.return_first_word();
    println!("first word is : {first_word}");
}
