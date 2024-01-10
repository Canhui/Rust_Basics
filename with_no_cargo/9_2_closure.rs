/* 
A Brief Introduction:
    1. usage of rust closure
*/
fn main() {
    let outer_var = 42;
    
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;
    let one = || 1;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    println!("closure returning one: {}", one());

}