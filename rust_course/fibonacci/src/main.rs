fn main() {
    let n: i32 = 4;
    println!("The {} element of fibonacci is: {}", n, recursive_fib(n))
}

fn recursive_fib(n: i32) -> i32 
{
    if n <= 1
    {
        return 1;
    }
    return recursive_fib(n - 1) + recursive_fib(n - 2);
}