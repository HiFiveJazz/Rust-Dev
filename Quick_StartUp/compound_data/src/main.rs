fn main() {
    //&str and String
    let fixed_str: &str = "Fixed length string";
    let mut flexible_str = String::from("This string will grow");
    flexible_str.push('s');
    println!("{flexible_str}");

    // Arrays
    let mut array_1 = [1,2,3,4,5];
    // let num = array_1[3];
    println!("{:?}", array_1);

    //Vectors
    let vec_1 = vec![4,5,6,8,9];

    // Tuples
    let my_info = ("Salary", 40_000, "Age", 40);
    let salary_info = my_info.1;
    let (salary, salary_value, age, age_value) = my_info;

}
