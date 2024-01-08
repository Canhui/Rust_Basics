use std::fmt;

/* 
A Brief Introduction:
    1. Override fmt::Display.to_string() method for Circle
*/
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    let str = circle.to_string();
    println!("{}", str);
}