mod todo;

use crate::todo::Todo;
use std::io::Error;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an actions");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{}, {}", action, item);

    let mut todo = Todo::new().expect("DB initialisation failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved!"),
            Err(msg) => println!("An error occured: {}", msg),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the todo list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved!"),
                Err(msg) => println!("An error occured: {}", msg),
            },
        }
    }
}
