/* 
A Brief Introduction:
    1. Usage of variable types
*/ 
fn main(){
    let a_logical_var: bool = true;
    let a_float_var: f64 = 1.0;
    let an_integer_var = 7;
    let mut a_mutable_var = 12;
    println!("a_logical_var={}, a_float_var={}", a_logical_var, a_float_var);
    println!("an_integer_var={}, a_mutable_var={}", an_integer_var, a_mutable_var);
    a_mutable_var = 4294967296i64;
    println!("a_mutable_var={}", a_mutable_var);
}