use std::io;
fn main() {
    loop {
        println!("enter no of terms");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        match input.trim().parse::<u32>() {
            Ok(nth_term) => {
                let mut t1 = 0;
                let mut t2 = 1;
                let mut next_term = t1 + t2;
                println!("below are fibonacci series");
                if nth_term < 3 {
                    println!("{}", t1);
                    println!("{}", t2);
                } else if nth_term >= 3 {
                    println!("{}", t1);
                    println!("{}", t2);
                
                    for _ in 3..=nth_term {
                        println!("{}", next_term);
                        t1 = t2;
                        t2 = next_term;
                        next_term = t1 + t2;
                    }
                }
                break;
            }
            Err(_) => {
                println!("please enter number");
            }
        }
        
    }
}