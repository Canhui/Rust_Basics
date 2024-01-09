/* 
A Brief Introduction:
    1. construct your own rust tuple
*/
fn main() {
    let triple = (0, -2, 3);
    println!("Basic information about {:?}", triple);

    // match can be used to destructure a tuple
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _      => println!("It doesn't matter what they are"),
    }
}