const BASE_VALUE: i32 = 32;
fn main() {
    //Ternary?
    let ternary: i32 = if true { 32 } else { 64 };

    // Basic usage
    if (BASE_VALUE == 32) 
    {
        println!("The base value realy is {}", BASE_VALUE);
    }
    else if (BASE_VALUE != 32) // I hate elses statements
    {
        println!("The base value realy is {}", BASE_VALUE);
    }
    else 
    {
        println!("What the fox says? thinininimmm");
    }

    // Switch case statements
    match (BASE_VALUE)
    {
        32 => println!("Dammit, is really {}!", BASE_VALUE),
        64 => println!("This never catches!"),
        _ =>  println!("Oh yeah, default case!"),
    }
}
