fn main(){

    //Unlike arrays, tuples can store different datatypes together
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00); //Creating a tuple
    println!("Second tuple item: {}", my_tuple.1);
}