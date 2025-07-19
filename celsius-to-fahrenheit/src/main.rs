use std::io;
fn main() {
    loop {
        println!("enter degree in celsius: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input from user");

       match input.trim().parse::<f64>() {
        Ok(celsius) => {
            let fahrenheit = (celsius * 9.0/5.0) + 32.0;
            println!("result in fahrenheit: {:.2}", fahrenheit);
            break;
        }
        Err(_) => {
            println!("given input is not a number");
        }
       } 
    }
}