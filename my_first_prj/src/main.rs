use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter any number");
    let number = rand::thread_rng().gen_range(1..=100);
    let mut checker = 0;
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You have entered {}", input);

        match input.cmp(&number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
        if checker == 10 {
            println!("You exceeded!");
            break;
        }

        checker += 1;
    }
    println!("The number was {}", number);
}
