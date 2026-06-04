fn main() 
{
    let mut value: i32 =
    {
         let x: i32 = 3;
         x + 8
    };

    println!("Changed value: {}", with_return(&mut value));
    println!("Calc value: {}", calc(2, 3));
}

fn calc(x: i32, y: i32) -> i32
{
    x.pow(2) + 4 * y
}

fn with_return(i: &mut i32) -> i32
{
    let k: i32 = *i + 8;
    return k;
}