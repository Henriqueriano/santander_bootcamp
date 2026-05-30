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
        println!("For loop, the last is not included: {}", num);
    }
}
