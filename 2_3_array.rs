use std::mem;

fn main(){
    let array_x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Memory size of the array: {} bytes", mem::size_of_val(&array_x));
    println!("First element of the array: {}", array_x[0]);
    println!("Second element of the array: {}", array_x[1]);
    println!("Array {:?}", array_x)
}
