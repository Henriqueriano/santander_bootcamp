fn main() {
    let mut x: i32 = 32;
    let &mut y: &i32 = &x;

    println!("Here, {:p} exists.", y);

    x = 12;
    // println!("Heres, raises {:p} raises error.", y);
    // Raises an error because the x value changes, now, borrow checker don't compile.
}
