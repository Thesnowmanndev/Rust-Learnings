// General Messages
pub fn print_greeting() {
    println!("Welcome to the Work Roster application.");
}

pub fn print_exit_text() { println!("Exiting."); }

pub fn print_number_of_employees(n_employees: &str) {
    println!("There are a total of {} employees in the database.", n_employees);
}

// Employee Messages
pub fn print_employee_added(rank: &str, f_name: &str, l_name: &str) {
    println!("{} {} {} added.", rank, f_name, l_name);
}

pub fn print_employee_number(e_number: &str) { println!("Employee Number: {}", e_number); }

pub fn print_flight_control_course(fc_course: &str) {
    println!("Qualified on Flight Controls: {}", fc_course);
}

pub fn print_doors_course(dr_course: &str) { println!("Qualified on Doors: {}", dr_course); }

pub fn print_cddar_course(cddar_course: &str) {
    println!("Qualified on CDDAR course: {}", cddar_course);
}

pub fn print_high_lift(high_lift: &str) {
    println!("Qualified on High Lift vehicles: {}", high_lift);
}

pub fn print_crane(crane: &str) { println!("Qualified on Crane vehicles: {}", crane); }

pub fn print_forklift(forklift: &str) { println!("Qualified on Forklift vehicles: {}", forklift); }

pub fn print_tow(tow: &str) { println!("Qualified on Tow vehicles: {}", tow); }