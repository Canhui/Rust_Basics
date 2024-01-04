fn main(){
    let decimal: f32 = 3.14;
    
    let integer;
    integer = decimal as u16; // casting from decimal to integer

    println!("decimal = {}", decimal);
    println!("integer = {}", integer);
}