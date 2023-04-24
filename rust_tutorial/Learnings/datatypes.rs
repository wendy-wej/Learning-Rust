fn datatypes(){
    //Unsigned Integer: u8, u16, u32, u64, u128, usize
    //signed Integer: i8, i16, i32, i64, i128, isize

    let _bool_checker:bool =true;
    let my_grade = 'A'; //remeber that characters take single quotation!

    let num_1:f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.1111111111111111); //1.2222223

    let num_2: f64 = 1.111111111111111;
    println!("f32: {}", num_2 + 0.1111111111111111); // 1.222222222222222

    let num_3: u32 = 45;
    let num_4:u32 = 55;
    println!("5 + 4 = {}", num_3+num_4); //Other operators include: '+', '-', '*', '/', '%'
}