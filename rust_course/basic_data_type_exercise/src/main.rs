/*
    Recieve two years argument and returns the age.
*/
fn age_calc(year_one: i16, year_two: i16) -> i16 
{
    year_two - year_one
}

fn main() {
    let year_one: i16 = 2003;
    let year_two: i16 = 2026;

    println!("Your age is: {}", age_calc(year_one, year_two));
    // output : 23
}
