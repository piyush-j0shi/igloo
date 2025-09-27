trait LibrarySystem {
    fn title(&self) -> &str;
    fn info(&self) -> String;
}

struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

struct DVD<'a> {
    title: &'a str,
    director: &'a str,
}

impl<'a> LibrarySystem for Book<'a> {
    fn title(&self) -> &str {
        self.title
    }

    fn info(&self) -> String {
        self.author.to_string()
    }
}

impl<'a> LibrarySystem for DVD<'a> {
    fn title(&self) -> &str {
        self.title
    }

    fn info(&self) -> String {
        self.director.to_string()
    }
}

fn itearte_over<T: LibrarySystem>(value: T) {
    let title = value.title();
    let info = value.info();

    println!("title : {title}");
    println!("info is : {info}");
}

fn main() {
    let book1 = Book {
        title: "book1",
        author: "author1",
    };
    let book2 = Book {
        title: "book2",
        author: "author2",
    };
    let book3 = Book {
        title: "book3",
        author: "author3",
    };

    let collect = vec![book1, book2, book3];

    for values in collect {
        itearte_over(values);
    }
}
