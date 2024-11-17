// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000; // This mutation works because y is not valid anymore  since L16
    /*
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references
    "Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value"
     */
    assert_eq!(x, 1200);
}
