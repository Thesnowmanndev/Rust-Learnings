use input_validation::{get_input, get_bool};

struct Employee {
    rank: String,
    last_name: String,
    first_name: String,
    employee_number: u32,
    flight_controls_course: bool,
    doors_course: bool,
    cddar_course: bool,
    high_lift_vehicle: bool,
    crane_vehicle: bool,
    forklift_vehicle: bool,
    tow_vehicle: bool,
}

fn main() {
    let mut run_application = true;
    let mut number_of_employees : u8 = 0;
    let mut employee_vector: Vec<Employee> = Vec::new();

    while run_application {
        greet();

        if number_of_employees == 0 {
            let new_employee: Employee = create_new_employee();
            employee_vector.push(new_employee);
            number_of_employees += 1;
        } else {
            let number_of_employees_str = number_of_employees.to_string();
            println!("There are a total of {} employees in the database.", number_of_employees_str);
        }

        run_application = exit();
        if run_application == false {
            break;
        }
    }
}

impl Employee {
    fn set_rank(&mut self) { self.rank = get_input("What is the employees rank? "); }

    fn set_last_name(&mut self) {
        self.last_name = get_input("What is the employees last name? ")
    }

    fn set_first_name(&mut self) { self.first_name = get_input("What is the employees first name? "); }

    fn set_employee_number(&mut self) {
        let emp_num_string: String = get_input("What is the employees number? ");
        let emp_num: u32 = emp_num_string.trim().parse().unwrap_or_else(|_| 0);
        self.employee_number = emp_num;
    }

    fn set_flight_controls_course(&mut self) {
        self.flight_controls_course = get_bool("Has the employee attended the flight controls course? (y/n) ");
    }

    fn set_doors_course(&mut self) {
        self.doors_course = get_bool("Has the employee attended the doors course? (y/n) ");
    }

    fn set_cddar_course(&mut self) {
        self.cddar_course = get_bool("Has the employee attended the CDDAR course? (y/n) ");
    }

    fn set_high_lift_vehicle(&mut self) {
        self.high_lift_vehicle = get_bool("Is the employee qualified on high lift vehicles? (y/n) ");
    }

    fn set_crane_vehicle(&mut self) {
        self.crane_vehicle = get_bool("Is the employee qualified on crane operations? (y/n) ");
    }

    fn set_forklift_vehicle(&mut self) {
        self.forklift_vehicle= get_bool("Is the employee qualified on forklift vehicles? (y/n) ");
    }

    fn set_tow_vehicle(&mut self) {
        self.tow_vehicle = get_bool("Is the employee qualified on tow vehicles? (y/n) ");
    }
}

fn create_new_employee() -> Employee {
    let mut new_employee = Employee {
        rank: "".to_string(),
        last_name: "".to_string(),
        first_name: "".to_string(),
        employee_number: 0,
        flight_controls_course: false,
        doors_course: false,
        cddar_course: false,
        high_lift_vehicle: false,
        crane_vehicle: false,
        forklift_vehicle: false,
        tow_vehicle: false,
    };

    new_employee.set_rank();
    new_employee.set_first_name();
    new_employee.set_last_name();
    new_employee.set_employee_number();
    new_employee.set_flight_controls_course();
    new_employee.set_doors_course();
    new_employee.set_cddar_course();
    new_employee.set_high_lift_vehicle();
    new_employee.set_crane_vehicle();
    new_employee.set_forklift_vehicle();
    new_employee.set_tow_vehicle();

    println!("{} {} {} added.", new_employee.rank, new_employee.first_name, new_employee.last_name);
    println!("Employee Number: {}", new_employee.employee_number.to_string());
    println!("Qualified on Flight Controls: {}", new_employee.flight_controls_course.to_string());
    println!("Qualified on Doors: {}", new_employee.doors_course.to_string());
    println!("Qualified on CDDAR course: {}", new_employee.cddar_course.to_string());
    println!("Qualified on High Lift vehicles: {}", new_employee.high_lift_vehicle.to_string());
    println!("Qualified on Crane vehicles: {}", new_employee.crane_vehicle.to_string());
    println!("Qualified on Forklift vehicles: {}", new_employee.forklift_vehicle.to_string());
    println!("Qualified on Tow vehicles: {}", new_employee.tow_vehicle.to_string());

    new_employee
}

fn greet() {
    println!("Welcome to the Work Roster application.");
}

fn exit() -> bool {
    let mut _stay_open = false;
    let confirm = get_bool("Do you want to continue running the application? (y/n) ");

    if confirm {
        _stay_open = true
    } else {
        println!("Exiting.");
        _stay_open = false;
    }

    _stay_open
}