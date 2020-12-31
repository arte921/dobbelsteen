use rand::Rng;
use std::env;

fn main() {
    let mut rng = rand::thread_rng();
    let args: Vec<String> = env::args().collect();
    // let max: u32 = args[0].trim().parse();

    let max = match args[1].parse::<u32>() {
        Ok(max) => max,
        Err(_) => 6
    };

    println!("{}", rng.gen::<u32>() % max + 1);
}
