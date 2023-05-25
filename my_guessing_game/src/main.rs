use std::{io, num}; //IO library helps with input output operations and is derived from the standard library(std)
use rand::Rng; //Rng is a trait that defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use std::cmp::Ordering;
//cargo doc --open build documentation provided by all your dependencies locally and open it in your browser.

fn main() {
    println!("Guess the number!");
   
    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("Please input your guess.");
        

        let mut guess = String::new(); //mut makes the variable mutable, by default variables are immutable in rust
        // "::" lets us know that new is an associated function of the String type. 
        //An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.
        
        

        io::stdin()
            .read_line(&mut guess) //read_line takes whatever the user types into standard input and places that into a string
            .expect("Failed to read line"); //read_line returns a value of type io::Result. Result types are enumerations, often referred to as enums. An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.
        
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // {} is a placeholder for the value of the variable guess

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;}
        }
    }


}
