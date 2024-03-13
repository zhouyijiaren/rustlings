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
    let mut name = String::from("Jill Smith");
    let mut title = String::from("Fish Flying");
    let book = Book { author: &mut name, title: &mut title };

    println!("{} by {}", book.title, book.author);
}
