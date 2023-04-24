#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");

    // All variables in rust are immutable by default. This is because it eliminates the state change of each variables and improves debugging.
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Didn't Receive Input"); //read_line generates two outcomes: 'OK" and "Error" to validate if it's received input
    println!("Hello, {}! {}", name.trim_end(), greeting); //the trim_end() gets rid of the newline created after the user inputs their name
}
