fn main(){
    println!("1+2 = {}", 1u32+2);
    println!("1-2 = {}", 1i32-2);
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // binary operation AND
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // binary operation OR
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // binary operation XOR
    println!("1 << 5 is {}", 1u32 << 5); // bitwise operation, from 000001 -> 100000
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // bitwise operation, from 10000000 -> 100000
}

