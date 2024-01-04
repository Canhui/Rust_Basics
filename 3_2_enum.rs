fn main(){
    #[derive(Debug)]
    enum Direction{
        North,
        East,
        South,
        West
    }

    let north = Direction::North;
    let east = Direction::East;
    let south = Direction::South;
    let west = Direction::West;

    println!("{:?}", north);
    println!("{:?}", east);
    println!("{:?}", south);
    println!("{:?}", west);
}