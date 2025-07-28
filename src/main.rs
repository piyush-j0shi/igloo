use std::io;

#[derive(Debug)]
enum Status {
    Bought,
    NotBought,
}

#[derive(Debug)]
struct Item {
    name: String,
    status: Status,
}

fn read_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    user_input.trim().to_string()
}

fn add_item(null_items: &mut Vec<Item>) {
    println!("enter the product you want to add");
    let product_name = read_input();
    let lowercase_productname = product_name.to_lowercase();

    if null_items
        .iter()
        .any(|s| s.name.to_lowercase() == lowercase_productname)
    {
        println!("item already exists");
        // return null_items;
    } else {
        let items = Item {
            name: product_name,
            status: Status::NotBought,
        };

        null_items.push(items);
        println!("item added");

        // null_items
    }
}

fn remove_item(items_list: &mut Vec<Item>) {
    if items_list.is_empty() {
        println!("there are not items")
    } else {
        view_item(&items_list);
        println!("enter the item name you want to remove");
        let removed_item = read_input();

        if let Some(index) = items_list
            .iter()
            .position(|index| index.name.to_lowercase() == removed_item)
        {
            items_list.remove(index);
            println!("item removed");
        } else {
            println!("no intem found");
        }
    }
}

fn view_item(some_items: &Vec<Item>) {
    if some_items.is_empty() {
        println!("there are no items, add some items to view items")
    }

    for (index, item) in some_items.iter().enumerate() {
        println!(
            "item no : {} | item name : {:?} | item status : {:?}",
            index, item.name, item.status
        );
    }
}

fn buy_items(added_items: &mut Vec<Item>) {
    if added_items.is_empty() {
        println!("there are no items at this moment");
    } else {
        view_item(&added_items);
        println!("enter the item name you want to buy");
        let buying_item = read_input().trim().to_lowercase();

        if let Some(item) = added_items
            .iter_mut()
            .find(|item| item.name.to_lowercase() == buying_item)
        {
            item.status = Status::Bought;
            println!("item marked as bought");
        } else {
            println!("item no found");
        }
    }
}

fn main() {
    let mut shopping_list: Vec<Item> = Vec::new();

    loop {
        println!("\n");
        println!("type add to add new items | type buy to but items | type view to view items | type remove to remove an item |press any key to exit and view items");

        // println!("type anything");
        // let input = read_input();
        // println!("you typed : {}", input);

        // let first_item = add_item(&mut shopping_list);
        // println!("item is : {:?}", first_item);

        let new_input = read_input().trim().to_lowercase();
        if new_input == "add" {
            add_item(&mut shopping_list);
        } else if new_input == "buy" {
            buy_items(&mut shopping_list);
        } else if new_input == "view" {
            view_item(&shopping_list);
        } else if new_input == "remove" {
            remove_item(&mut shopping_list);
        } else {
            break;
        }
    }
}
