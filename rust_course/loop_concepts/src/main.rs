fn main() {
    loop 
    {
        println!("Do something!");
        if true {break;} 
        /*
         Breaks stops the loop,
        "continue" pass all las statemetns.
        */
    }
   
    #[warn(while_true)] 
    while true 
    {
        println!("Also, do semthing again!");
        break;
    }

    for num in 1..4_i32 
    { 
        /*
         Note, the _i32 is the type of "num" var,
         and, if I want <= at 4, I use 1..=4 instead 1..4
        */
        println!("For loop, the last is not included: {}", num);
    }
}
