fn conditionals(){
    // If-else-statement
    let age: i32 = 8;
    if(age>=18) && (age<=65){
        println!("Yayy, you're eligible for the labour workforce")
    } else if(age>65){
        println!("Yup, you should be retired by now!")
    }else{
        println("What are you doing here, you should be in school!")
    }

    //Tenary operator
    let mut my_age = 47;
    let can_join_army:bool =if my_age >= 18{true
    }else{
        false
    };
    println!("Can you join the military: {}", can_join_army);

    
}