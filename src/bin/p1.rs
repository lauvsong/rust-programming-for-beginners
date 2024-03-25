// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

 use std::collections::HashMap;
 use Command::*;
 use std::io;
 use std::process;

 #[derive(Clone)]
 struct Bill {
    name: String,
    amount: i32
 }

 struct Bills {
    inner: HashMap<String, Bill> // key: name
 }

 impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn get(&self, name: &str) -> Option<&Bill> {
        self.inner.get(name)
    }

    fn add(&mut self, bill: &Bill) {
        self.inner.insert(bill.name.clone(), bill.clone());
    }

    fn view(&self) -> &HashMap<String, Bill> {
        &self.inner
    }

    fn remove(&mut self, name: &str) -> Result<(), String> {
        match self.inner.remove(name) {
            Some(_) => Ok(()),
            None => Err("Error while removing bill".to_owned())
        }
    }

    fn edit(&mut self, name: &str, bill: &Bill) -> Result<(), String> {
        self.remove(name)?;
        self.add(bill);
        Ok(())
    }
 }

 enum Command {
    Add,
    View,
    Remove,
    Edit,
    Exit
 }

 impl Command {
    fn new(input: &str) -> Option<Command> {
        let command = input.trim().to_lowercase();
        match command.as_str() {
            "1" => Some(Add),
            "2" => Some(View),
            "3" => Some(Remove),
            "4" => Some(Edit),
            "5" => Some(Exit),
            _ => None
        }
    }
 }

 fn get_input() -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter your data again");
    }
    buffer.trim().to_lowercase()
 }

 fn get_command() -> Command {
    manual_print();
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        print!("try again");
    }

    match Command::new(&buffer) {
        Some(command) => command,
        None => get_command()
    }
 }

 fn get_amount_input() -> i32 {
    loop {
        let input = get_input();
        let parsed_input = input.parse::<i32>();

        match parsed_input {
            Ok(num) => return num,
            _ => {
                println!("invalid amount");
                continue; 
            }
        }
    }
 }

 fn get_billname_input(bills: &Bills) -> String {
    loop {
        let input = get_input();
        if !is_billname_exists(bills, input.as_str()) {
            println!("Bill name not found. Check your name exists and type again.");
            continue;
        }

        return input;
    } 
 }

 fn is_billname_exists(bills: &Bills, name: &str) -> bool {
    match bills.get(name) {
        Some(_) => true,
        None => false
    }
}

 fn action(command: &Command, bills: &mut Bills) {
    match command {
        Add => add(bills),
        View => view(bills),
        Remove => remove(bills),
        Edit => edit(bills),
        Exit => process::exit(1)
    }
 }

 fn add(bills: &mut Bills) {
    println!("");

    println!("bill name:");
    let name = get_input();

    println!("bill amount:");
    let amount = get_amount_input();

    bills.add(
        &Bill{
            name: name,
            amount: amount
        }
    );
 }

fn view(bills: &Bills) {
    let mut index = 0;
    for (name, bill) in bills.view().iter() {
        index += 1;
        println!("{:?}. name: {:?}, amount: {:?}", index, name, bill.amount);
    }
}

fn remove(bills: &mut Bills) {
    println!("bill name:");
    let mut name = get_billname_input(bills);

    loop {
        if !is_billname_exists(bills, name.as_str()) {
            return 
        }

        match bills.remove(name.as_str()) {
            Ok(_) => break,
            _ => println!("Bill name not found. Check your name exists and type again.")
        }

        name = get_input();
    }
}

fn edit(bills: &mut Bills) {
    println!("bill name:");
    let name = get_billname_input(bills);

    println!("bill amount:");
    let amount = get_amount_input();

    let edit_result = bills.edit(
        name.as_str(),
        &Bill{
            name: name.to_owned(),
            amount: amount
        }
    );

    match edit_result {
        Err(_) => println!("Failed to edit bill.bill name not found."),
        _ => ()
    }
}

fn manual_print() {
    println!("");
    println!("Type action number you want to execute.");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bills");
    println!("4. Edit bill");
    println!("");
}

fn main() {
    let mut bills = Bills::new();
    println!("=====Bill Manager=====");
    
    loop {
        let command = get_command();
        action(&command, &mut bills);
    }
}
