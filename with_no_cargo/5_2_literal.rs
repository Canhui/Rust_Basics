/* 
A Brief Introduction:
    1. If no type specified, then rust will use i32 for integers and f64 for floating-point numbers.
*/
fn main(){
    let x = 1u8; // suffixed literals, their types are known
    let y = 2u32;
    let z = 3f32;

    let i = 1; // unsuffixed literals, their types depend on how they are used
    let f = 1.0;  

    println!("size of x in bytes: {}", std::mem::size_of_val(&x)); // size of a variable in bytes
    println!("size of y in bytes: {}", std::mem::size_of_val(&y));
    println!("size of z in bytes: {}", std::mem::size_of_val(&z));
    println!("size of i in bytes: {}", std::mem::size_of_val(&i));
    println!("size of f in bytes: {}", std::mem::size_of_val(&f));
}