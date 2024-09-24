fn main() {
    let unsigned_num: u8 = 5; // u16, u32, u64, u128

    let signed_num: i8 =5; //i16, i32, i64, u128

    let float_num: f32 = 5.0; //f64

    // Platform specific integers
    let arch_1: isize = 5;
    let arch_2: usize = 5;

    //Characters
    let char = 'a';

    //Boolean
    let b: bool = true;

    // Type aliasing
    type Age = u8;
    let peter_age: Age = 42;

    // Type conversion
    let a = 10;
    let b = a as f64;

    // RANGES
    //u8 -> 0 - 255
    //u16 -> 0 - 65,535
    //u32 -> 0 - 4,294,967,295
    //u64 -> 0 - 18,446,774,073,709,551,615

    //i8 -> -128 - 127
    //i16 -> -32,768 - 32,767
    //i32 -> -2,147,483 - 2,147,483
    
    //f32 -> 7 digits of precision
    //f64 -> 15 digits of precision

}
