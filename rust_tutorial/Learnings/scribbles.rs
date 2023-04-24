fn scribbles(){

    //THIS IS FOR RANDOM CODE THAT DOESN'T FIT IN ANYWHERE
    const ONE_MIL: u32 = 1_000_000;
    const PI:f32 = 3.141592;
    let age:&str = "47"; // strings use double quotes while characters use single quotes
    let mut age:u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age +1;

    println!("I'm {} and I want ${}", age, ONE_MIL);

    //Unsigned Integer: u8, u16, u32, u64, u128, usize
    //signed Integer: i8, i16, i32, i64, i128, isize 
}