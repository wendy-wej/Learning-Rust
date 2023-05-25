
fn main(){
    let mut our_str = String::new();
    our_str.push(ch);
    our_str.push_str("word");

    for word in our_str.split_whitespace(){
        print!("{}", word);
    }
    let st2 = our_str.replace("A", "Another");
    print!("{}", st2 );

}