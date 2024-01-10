/* 
A Brief Introduction:
    1. Usage of rust while functions
*/
fn main() {
    let mut n = 1; // a mutable variable
    while n < 100{
        if n % 15 == 0{
            println!("case a");
        } else if n % 3 == 0{
            println!("case b");
        } else if n % 5 == 0{
            println!("case c");
        } else{
            println!("case d");
        }
        n += 1;
    }
}