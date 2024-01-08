/* 
A Brief Introduction:
    1. Usage of rust alias.
*/
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main(){
    let ns: NanoSecond = 5 as U64;
    let ic: Inch = 2 as U64;    
    println!("{} nanoseconds + {} inches = {} units", ns, ic, ns+ic); 
}