mod library {
    mod book {
        pub struct Book {
            title: String,
            author: String,
        }

        impl Book {
            fn new(title: &str, author: &str) -> Self {
                Self {
                    title: title.to_string(),
                    author: author.to_string(),
                }
            }
        }
    }
    mod bookshelf {
        struct Bookshelf {
            // books: Vec<crate::library::book::Book>,
            books: Vec<super::book::Book>,
        }

        impl Bookshelf {
            fn new() -> Self {
                Self { books: Vec::new() }
            }
        }
    }
}
