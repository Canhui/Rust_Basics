#![allow(unreachable_code, unused_labels)]
/* 
A Brief Introduction:
    1. Usage of rust loop functions
*/

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer; // This breaks the outer loop
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}