// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }


fn borrows_var(some_string: &String) {
    println!("{some_string}");
}



fn main() {
    let s = String::from("hello");
    // takes_ownership(s);
    borrows_var(&s);
}