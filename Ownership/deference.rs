fn main (){
    let x = 5;
    let y = &x;
    println!("Value of x: {}", x);
    println!("Value of y: {:p}", y);
    println!("Value of *y: {}", *y);
    println!("Value of &x: {:p}", &x);
    println!("Value of *&x: {}", *&x);
}
