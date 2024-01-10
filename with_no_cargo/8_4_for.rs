/* 
A Brief Introduction:
    1. Usage of rust for functions
*/
fn main() {
    for n in 1..100{
        if n % 15 == 0{
            println!("case a");
        } else if n % 3 == 0{
            println!("case b");
        } else if n % 5 == 0{
            println!("case c");
        } else{
            println!("case d");
        }
    }
}