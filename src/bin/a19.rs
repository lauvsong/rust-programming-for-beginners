// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;


fn main() {

    let mut store = HashMap::new();
    store.insert("Chair", 5);
    store.insert("Bed", 3);
    store.insert("Table", 2);
    store.insert("Couch", 0);

    let mut total_stock = 0;

    for (name, stock) in store.iter() {
        total_stock += stock;
        match stock {
            0 => println!("out of stock"),
            _ => println!("name: {:?}, stock: {:?}", name, stock)
        }
    }

    println!("total stock: {:?}", total_stock);
}
