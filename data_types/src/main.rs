/**
.primitive data types in rust
primitive data types are the basic data types or fundamental data types used to declare variables
scalar and compound
scalar data type:
a scalar data type is somehting that has a finite set of possible values following some scale i.e each value can be compared to any other value as either equal greater or less

while as a compound data type 
In computer science a composite data type or compound data type is any data type which can be constructed in a program using the programing language's 
primitive data types and other composite data types.
we have an array and a tuple
rust is a statically types language
we need to know implicitly the data type of the variable or explicitly
defined


*/
fn main() {
    let x: i32=2; //default i32 = let x =2
    ///:i8
    /// i16
    /// i32
    /// 164
    /// i128 bit integer signed any whole number and doesnt contain fractional or decimal part positive and negative integer
    
    let y: u32=973; //unsigned integers u64 u128
    let z:f32=10.9 // single precision default 
    let xx:f64=88.9 // double precision

    // boolean
    let statement:bool = true; // false 0 , true 1

    let letter : char= '9' //'.' 



    println!("Hello, world!");
}
