fn main() {
    let x = 3;
    let y = 4;
    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
)
}
fn add_5(num1: i32) -> i32 {
    num1 + 5
    }
fn add_3(num1: i32) -> i32{
    num1 + 3  
    }
fn times(num1: i32, num2: i32) -> i32 {
    num1 * num2

    }

