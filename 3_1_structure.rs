#[derive(Debug)]

/* 
A Brief Introduction:
    1. Usage of rust structure
*/ 
struct Person{
    name: String,
    age: u8
}

struct Point{
    x: f32,
    y: f32
}

fn main(){
    // Init the structure
    let name = String::from("Peter");
    let age = 27;
    let peter = Person{name, age};
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Print the structure
    println!("The first structure: {:?}", peter);
    println!("The second structure: ({}, {})", point.x, point.y);
}