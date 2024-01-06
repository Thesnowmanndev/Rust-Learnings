mod employee;
mod view;
mod view_model;

use employee::{
    Employee,
    create_new_employee,
};
use view::{
    print_greeting,
    print_number_of_employees,
};
use view_model::exit;


fn main() {
    let mut run_application = true;
    let mut number_of_employees : u8 = 0;
    let mut employee_vector: Vec<Employee> = Vec::new();

    while run_application {
        print_greeting();

        if number_of_employees == 0 {
            let new_employee: Employee = create_new_employee();
            employee_vector.push(new_employee);
            number_of_employees += 1;
        } else {
            let number_of_employees_str = number_of_employees.to_string();
            print_number_of_employees(&number_of_employees_str);
        }

        run_application = exit();
        if run_application == false {
            break;
        }
    }
}