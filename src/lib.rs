pub mod library;

fn function_1() {
    let shelf = library::bookshelf::Bookshelf::new();
}

fn function_2() {
    use library::bookshelf;
    let shelf = bookshelf::Bookshelf::new();
}
