fn main() {
    my_fn("This is my function!");
    let str = "Function call with a variable";
    my_fn(str);
    let b = multiplication(1,2);
    // println!("{b}");
    let c = basic_math(1, 2);
    println!("{:?}", c);
}

fn my_fn(s: &str) {
    println!("{s}")
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication");
    return num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
