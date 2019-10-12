extern crate rand;

use std::io;
use std::io::Write;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0, 10 + 1);

    loop {
        let mut input_buffer = String::new();
        print!("Your guess:> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Error in reading from STDIN");
        let input_number: i32 = input_buffer
            .trim()
            .parse()
            .expect("Impossible to parse the input into an integer");
        if input_number == random_number {
            println!("Yeaaah! You guess correctly!");
            return;
        }
        if input_number > random_number {
            println!("Your guess is too big.");
        }
        if input_number < random_number {
            println!("Your guess is too small.");
        }
    }
}
