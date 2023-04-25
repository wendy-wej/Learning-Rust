#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    

    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00); //Creating a tuple
    println!("Second tuple item: {}", my_tuple.1);
    

}
