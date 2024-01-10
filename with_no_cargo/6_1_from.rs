use std::convert::From;
#[derive(Debug)]

/* 
A Brief Introduction:
    1. From one data type to another data type
*/
struct Number{
    value_a: i32,
    value_b: i32,
}

impl From<i32> for Number{
    fn from(item_1: i32) -> Self{
        Number{value_a: item_1, value_b: item_1}
    }
}

fn main(){
    let num_from = Number::from(30);
    println!("My number is {:?}", num_from);
}