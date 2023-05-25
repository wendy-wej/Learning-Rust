fn main() {
    //Here 'message is not valid because it is not in scope
    let message ="Hello, world!"; //message is valid from here on
    //the scope of message is over

    let mut hello_world = String::from("Hello");  //String::from() requests for memory on the heap
    hello_world.push_str(", world!"); //push_str() appends a literal to a String
    println!("{}", hello_world); //This will print `Hello, world!`

    // rust calls a special function called 'drop' when a variable goes out of scope.
    // The drop function is where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    // Ways Variables and Data Interact: Move
    let s1 = String::from("hello");
    let s2 = s1; //s1 is no longer valid here because it was moved to s2
    //println!("{}, world!", s1); //this throws 



    //References and borrowing
    let s3 = String::from("hello");

    let len = calculate_length(&s3);

    println!("The length of '{}' is {}.", s3, len);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}
