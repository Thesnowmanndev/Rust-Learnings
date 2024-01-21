use retirement_cli::exit;

fn main() {
    let mut run_app = true;

    while run_app {
        run_app = exit();
    }
}

