/* 
A Brief Introduction:
    1. Usage of rust mutability variables
*/ 
fn main(){
    let immutable_binding = 1; // cannot mutate it
    let mut mutable_binding = 1; // mutate it
    
    println!("Before mutation: {}", immutable_binding);
    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", immutable_binding);
    println!("After mutation: {}", mutable_binding);
}