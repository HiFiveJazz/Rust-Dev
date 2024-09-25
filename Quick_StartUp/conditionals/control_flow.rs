fn main() {
    'outer: loop {
        println!("Simple Loop");
        break; 'outer;
    }

    let a = loop {
        break 5;
    };

    let vec = vec![1, 2 ,3 , 4, 5, 6, 7];

    for i in vec {
        println!("{i}");
    }

    let mut num: i32 = 0;
    while num < 10 {
        num = num + 1;
    }
}
