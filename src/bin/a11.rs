// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    number: i32
}

fn display_quantity(grocery: &Grocery) {
    println!("quantity: {:?}", grocery.quantity)
}

fn display_number(grocery: &Grocery) {
    println!("number: {:?}", grocery.number)
}

fn main() {
    let grocery = Grocery {
        quantity: 1,
        number: 2
    };

    display_quantity(&grocery);
    display_number(&grocery);
}
