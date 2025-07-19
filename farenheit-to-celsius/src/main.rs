use std::io;

fn main() {
    println!("enter Fahrenheit value: ");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("failed to read input from user");

    let fahrenheit:f64 = fahrenheit
        .trim()
        .parse()
        .expect("input is not number");

    let result = (fahrenheit - 32.0) * 5.0/9.0;

    println!("Degree in celsius: {:.4}", result);
}