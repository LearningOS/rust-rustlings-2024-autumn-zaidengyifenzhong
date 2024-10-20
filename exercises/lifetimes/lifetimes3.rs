// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");

    // 确保 `name` 和 `title` 的生命周期足够长，以便 `book` 可以安全地引用它们
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
