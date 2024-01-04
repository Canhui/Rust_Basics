fn main(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, "a_str", true);
    println!("First item at the long tuple: {}", long_tuple.0);
    println!("Second item at the long tuple: {}", long_tuple.1);

    let tuple_of_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuple: {:?}", tuple_of_tuple)
}