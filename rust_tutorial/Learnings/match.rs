use std::cmp::Ordering;

fn match_func(){
    //Match
    let age2: i32=43;
    match age2{
        1..=17 => println!("What are you doing here, you should be in school!"), //1 to 17 (seventeen inclusive)
        18..=40 => println!("Yayy, you're eligible for the labour workforce"), 
        65..=i32::MAX => println!("Heuii!! You should be in a retirement home...or deceased"), //65 to the max value of i32, done to avoid errors.
        _ => println!("Just an inbetweener") //similar to an else statement
    }; 

    let my_age = 19;
    let voting_age:i32 = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Too young"),
        Ordering::Greater => println!("Eligible to vote"),
        Ordering::Equal => println!("Just Old enough to vote")
    }

}