#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    

    let my_age = 6;
    let voting_age:i32 = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Too young"),
        Ordering::Greater => println!("Eligible to vote"),
        Ordering::Equal => println!("Just Old enough to vote")
    }
    

}
