use std::io;
fn main() {
    let mut s = String::new();

    println!("enter your name");

    io::stdin()
        .read_line(&mut s)
        .expect("failed to read line");

    greet(s);
}

fn greet(s: String) {
    println!("Welcome {}", s);
}