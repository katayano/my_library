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
        use super::book::Book;

        struct Bookshelf {
            // books: Vec<crate::library::book::Book>,
            books: Vec<super::book::Book>,
        }

        impl Bookshelf {
            pub fn new() -> Self {
                Self { books: Vec::new() }
            }

            // 本を追加するメソッド
            pub fn add_book(&mut self, book: Book) {
                self.books.push(book);
            }

            // タイトルで本を検索するメソッド
            pub fn search_books(&self, title_query: &str) -> &Vec<Book> {
                todo!("Implementend");
            }

            // 本を本棚から取り出すメソッド
            pub fn remobe_book(&mut self, book: &Book) -> Option<Book> {
                todo!("Implemented");
            }

            // 本棚の本をすべて取り出すメソッド
            pub fn take_all_books(&mut self) -> Vec<Book> {
                todo!("Implemented");
            }
        }
    }
}
