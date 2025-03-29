use super::book::Book;

pub struct Bookshelf {
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
    pub fn search_books_exact(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title == title_query)
            .collect()
    }

    // タイトル名の部分一致で本を検索するメソッド
    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title.contains(title_query))
            .collect()
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
