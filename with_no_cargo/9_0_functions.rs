/* 
A Brief Introduction:
    1. usage of functions in rust language
*/
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("it is divisible by 15");
    } else if is_divisible_by(n, 3) {
        println!("it is divisible by 3");
    } else if is_divisible_by(n, 5) {
        println!("it is divisible by 5");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn main() {
    fizzbuzz_to(100);
}