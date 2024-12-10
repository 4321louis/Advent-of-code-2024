use std::{env, fs};

use days::*;
mod days;

fn main() { 
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].clone())
        .expect("Should have been able to read the file");
    println!("{}",day4::solve_one(contents.clone()));
    println!("{}",day4::solve_two(contents));
}
