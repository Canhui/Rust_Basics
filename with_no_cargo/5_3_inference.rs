/* 
A Brief Introduction:
    1. Basic operations in other language could be a highlighted feature in secure rust, such as vector.
*/
fn main(){
    let element_one = 5u8;
    let element_two = 6u8;
    let mut vec = Vec::new();
    vec.push(element_one); 
    vec.push(element_two);  
    println!("{:?}", vec);
}