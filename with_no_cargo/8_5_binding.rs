/* 
A Brief Introduction:
    1. break down rust binding and construct your own rust binding
*/
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");
    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n             => println!("I'm an old person of age {:?}", n),
    }
}