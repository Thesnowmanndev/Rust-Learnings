use input_validation::{get_bool, get_input};

use crate::view::*;

pub(crate) struct Employee {
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

pub fn create_new_employee() -> Employee {
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

    let rank: String = new_employee.rank.to_string();
    let first_name: String = new_employee.first_name.to_string();
    let last_name: String = new_employee.last_name.to_string();
    let employee_number: String = new_employee.employee_number.to_string();
    let flight_controls_course_status: String = new_employee.flight_controls_course.to_string();
    let doors_course_status: String = new_employee.doors_course.to_string();
    let cddar_course_status: String = new_employee.cddar_course.to_string();
    let high_lift_vehicle_status: String = new_employee.high_lift_vehicle.to_string();
    let crane_vehicle_status: String = new_employee.crane_vehicle.to_string();
    let forklift_vehicle_status: String = new_employee.forklift_vehicle.to_string();
    let tow_vehicle_status: String = new_employee.tow_vehicle.to_string();

    print_employee_added(&rank, &first_name, &last_name);
    print_employee_number(&employee_number);
    print_flight_control_course(&flight_controls_course_status);
    print_doors_course(&doors_course_status);
    print_cddar_course(&cddar_course_status);
    print_high_lift(&high_lift_vehicle_status);
    print_crane(&crane_vehicle_status);
    print_forklift(&forklift_vehicle_status);
    print_tow(&tow_vehicle_status);

    new_employee
}