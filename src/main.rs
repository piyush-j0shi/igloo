trait Summarize {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Book {
    author: String,
    title: String,
    publicationdate: String,
}

#[derive(Debug)]
struct Movie {
    name: String,
    productionhouse: String,
    releasedate: String,
}

impl Summarize for Book {
    fn summarize(&self) -> String {
        format!(
            "{} by {} on {}",
            self.title, self.author, self.publicationdate
        )
    }
}

impl Summarize for Movie {
    fn summarize(&self) -> String {
        format!(
            "{} by {} on {}",
            self.name, self.productionhouse, self.releasedate
        )
    }
}

fn print_summary<T: Summarize>(value: T) -> String {
    value.summarize()
}

fn main() {
    let book = Book {
        author: "bookauthor".to_string(),
        title: "booktitle".to_string(),
        publicationdate: "bookpublicationdate".to_string(),
    };

    let movie = Movie {
        name: "moviename".to_string(),
        productionhouse: "movieproductionhouse".to_string(),
        releasedate: "moviereleaseddate".to_string(),
    };

    println!("{}", book.summarize());
    println!("{}", movie.summarize());

    let book_details = print_summary(book);
    println!("{}", book_details);
}
