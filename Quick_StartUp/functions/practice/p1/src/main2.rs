fn main() {
    let x = triple(double(5));
    let y = triple(x);
    println!("Answer: {}", y);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

