/* 
A Brief Introduction:
    1. Usage of rust mutable variables
*/ 
fn main(){
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        println!("value of the mutable integer within scope: {}", _mutable_integer);
        // _mutable_integer = 50; // ERROR: failed to modify it due to freezing in this scope now
        println!("value of the mutable integer within scope: {}", _mutable_integer);
    }
    _mutable_integer = 60;
    println!("value of the mutable integer outside scope: {}", _mutable_integer);
}