struct Car{
    owner: String, 
    year: u32,
    fuel_level: f32,
    price: u32, 
}

fn main () {
    let my_car = Car {
        owner: String::from("ABC"), 
        year: 2010.
        fuel_level: 0.0,
        price: 
    }
    let point_2D = (1,3);
    let point_3D = (4,10,13);

    struct Point_2D(i32,i32);
    struct Point_3D(i32,i32,i32);

    let point1 = Point_2D(1, 2);
    let point2 = Point_3D(1, 2, 1);

    //Unit Struct
    struct ABC;
}

fn display_car_info(car: &Car) {
    println!(
        "Owner: {}, Year: {}, Fuel Level: {}", 
        car.owner, car.price, car.fuel_level
    )
}
