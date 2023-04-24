fn random(){
    let random_num:i32 = rand::thread_rng().gen_range(1..101); //generates number between 1 to 100
    println!("Random number generated {}", random_num)
}