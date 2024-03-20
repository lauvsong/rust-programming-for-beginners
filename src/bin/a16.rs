// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>
}

fn main() {

    let students = vec![
        Student {
            name: "Mary".to_owned(),
            locker: Some(1)
        },
        Student {
            name: "Alice".to_owned(),
            locker: None
        },
        Student {
            name: "Bob".to_owned(),
            locker: Some(3)
        }
    ];

    for student in &students {
        println!("number: {:?}", student.name);
        match student.locker {
            Some(num) => println!("locker: {:?}", num),
            None => println!("no locker assigned")
        }
    }
}
