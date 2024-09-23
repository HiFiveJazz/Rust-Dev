fn main() {
    // Definition
    let x = 10.0;
    println!("x is: {x}");

    // Mutability
    let mut y: i32 = 5;
    y = 10;

    // Scope
    {
        let z = 32;
    }
    // let s = z;

    //Shadowing
    let t = 10;
    let t = t + 10;
    println!("t is {t}");

    let u = 3;
    let u = 3.0;

    let v = 30;
    {
        let v = 40;
        println!("Inner v is: {v}");
    }
    println!("v is: {v}");

    // Constants
    const MAX_VALUE: u32 = 100;
}
