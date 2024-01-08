/* 
A Brief Introduction:
    1. Usage of rust shadowed variables
*/ 
fn main(){
    let shadowed_binding = 1;
    
    {
        println!("Before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abc";
        println!("Shadowed within inner block: {}", shadowed_binding);
    }
    println!("Shadowed outside inner block: {}", shadowed_binding);

    let shadowed_binding = "cbd";
    println!("Shadowed outside inner block: {}", shadowed_binding);
}