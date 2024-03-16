// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Chocolate,
    Vanilla,
    Mix
}

struct Drink {
    flavor: Flavor,
    fluid: f64
}

fn print_flavor(flavor:Flavor) {
    match flavor {
        Flavor::Chocolate => println!("chocolate"),
        Flavor::Vanilla => println!("vanilla"),
        Flavor::Mix => println!("mix")
    }
}

fn print_drink(drink:Drink) {
    print_flavor(drink.flavor);
    println!("{:?}", drink.fluid);
}

fn main() {
    let chocolate = Drink {
        flavor: Flavor::Chocolate,
        fluid: 6.0
    };
    print_drink(chocolate);

    let vanilla = Drink {
        flavor: Flavor::Vanilla,
        fluid: 10.0
    };
    print_drink(vanilla);

    let mix = Drink {
        flavor: Flavor::Mix,
        fluid: 12.0
    };
    print_drink(mix);
}
