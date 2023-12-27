use input_validation::get_bool;

use crate::view::print_exit_text;

pub fn exit() -> bool {
    let mut _stay_open = false;
    let confirm = get_bool("Do you want to continue running the application? (y/n) ");

    if confirm {
        _stay_open = true
    } else {
        print_exit_text();
        _stay_open = false;
    }

    _stay_open
}