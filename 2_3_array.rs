use std::mem;

/* 
A Brief Introduction:
    1. Usage of rust module, 
                    use std::mem; 
                    mem::size_of_val;
    Ref: https://doc.rust-lang.org/std/
*/ 
fn main(){
    let array_x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Memory size of the array: {} bytes", mem::size_of_val(&array_x));
    println!("First element of the array: {}", array_x[0]);
    println!("Second element of the array: {}", array_x[1]);
    println!("Array {:?}", array_x)
}