/*
    ownership ('dono', 'propietário' in pt-br), of the data.
*/

fn main() {
    let stack: i32 = 16; // This, goes to the stack section memory.
    let mut head: String = String::from("Hi, sayng 'hi!' from heap memory."); // This, goes to the heap.

    let copy_the_stack: i32 = stack; //This copies the value of stack not does a ref.

    // Printing the memory addres of copy_the_stack and the stack var in the memory.
    println!("Per copy:");
    println!("copy the stack {:p}", &copy_the_stack);
    println!("stack {:p}\n", &stack);

    let ref_base: i32 = 32;
    let access: &i32 = &ref_base;
    println!("Per ref:");
    println!("ref base {:p}", &ref_base);
    println!("stack {:p}\n", access);


}
