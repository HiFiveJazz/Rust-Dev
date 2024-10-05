struct Item {
    id: i32,
    title: String,
    year: i32,
    type_: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Book, 
    Magazine
}

fn display_item_info(item: &Item) {
    // for i in Item {
        println!("ID: {:?}", item.id);
        println!("Title: {:?}", item.title);
        println!("Year: {:?}", item.year);
        println!("Type: {:?}", item.type_);
        // println!("i value:{}, ID: {Item::id}, Title:")
    // }
}

fn main () {
    let rust_book = Item {
        id: 112351,
        title: String::from("Mario"),
        year: 2021,
        type_: ItemType::Book,
    };
    let rust_mag= Item {
        id: 112351,
        title: String::from("Mario Mag"),
        year: 2021,
        type_: ItemType::Magazine,
    };
    display_item_info(&rust_book);
    display_item_info(&rust_mag);
}
