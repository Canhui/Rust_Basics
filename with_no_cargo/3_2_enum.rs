#[derive(Debug)]

/* 
A Brief Introduction:
    1. Usage of rust enumerate
*/ 
enum Direction{
    North,
    East,
    South,
    West
}

fn main(){
    let north = Direction::North;
    let east = Direction::East;
    let south = Direction::South;
    let west = Direction::West;

    println!("{:?}", north);
    println!("{:?}", east);
    println!("{:?}", south);
    println!("{:?}", west);
}