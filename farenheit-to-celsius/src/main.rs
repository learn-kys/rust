fn main() {
    /* 
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
    */

    // improved code
    loop {
        println!("enter fahrenheit value: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");


        match input.trim().parse::<f64>() {
            Ok(fahrenheit) => {
                let celsius = (fahrenheit - 32.0) * 5.0/9.0;
                println!("Degree in celsius: {:.4}", celsius);
                break;
            }
            Err(_) => {
                println!("please enter a valid number");
            }
        }
    }
    
}