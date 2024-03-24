// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician
}

struct KeyCard {
    employee_type: EmployeeType,
    employed: bool
}

fn access_building(key_card: &KeyCard) -> Result<(), String> {
    match key_card.employed {
        false => return Err("access denied :: terminated employee".to_owned()),
        _ => ()
    }


    match key_card.employee_type {
        EmployeeType::Maintenance => Ok(()),
        EmployeeType::Manager => Ok(()),
        EmployeeType::Marketing => Ok(()),
        _ => Err("access denied :: not granted employee type".to_owned())
    }
}

fn print_access(key_card: &KeyCard) -> Result<(), String> {
    let result = access_building(key_card)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let key_card = KeyCard {
        employee_type: EmployeeType::Maintenance,
        employed: true
    };

    match print_access(&key_card) {
        Err(message) => println!("error :: {:?}", message),
        _ => ()
    }
}
