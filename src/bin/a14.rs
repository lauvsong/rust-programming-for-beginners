// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String
}

impl Person {
    fn printInfo(&self) {
        println!("name: {:?}", self.name);
        println!("color: {:?}", self.color);
    }
}

fn main() {

    let persons = vec![
        Person {
            age: 9,
            name: String::from("aa"),
            color: String::from("red")
        },
        Person {
            age: 10,
            name: String::from("bb"),
            color: String::from("blue")
        },
        Person {
            age: 11,
            name: String::from("cc"),
            color: String::from("green")
        }
    ];

    for person in &persons {
        if person.age <= 10 {
            person.printInfo();
        }
    }
}