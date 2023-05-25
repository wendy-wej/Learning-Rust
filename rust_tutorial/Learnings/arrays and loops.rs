fn learning_arrays(){
    //elements in an array must be of the same datatype.
    let arr_1 = [1,2,3,4,5,6,7,8,9];
    println!("First array item: {}", arr_1[0]); //prints 1
    println!("Length: {}", arr_1.len());

    let mut i:i32 = 0;

    //Loop
    loop{
        if arr_1[i] % 2 ==0{
            i +=1;
            continue;
        }
        if arr_1[i] == 9{
            break;
        }
        println!("Val :{}", arr_1[i]);
        i+=1;
    }

    //While loop
    while (i< arr_1.len()) {
        println!("Val :{}", arr_1[i]);
        i+=1
    }

    //For loop
    for j in arr_1.iter(){
        println!("Val: {}, val")
    }

}