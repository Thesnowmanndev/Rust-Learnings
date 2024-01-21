use console::Style;
use dialoguer::Confirm;

// Exit Logic
pub fn exit() -> bool {
    let mut c = true;
    let s = 
    format!("Do you want to {}?", Style::new()
                                        .green()
                                        .apply_to("continue"));
    let confirmation = Confirm::new()
        .with_prompt(s)
        .interact()
        .unwrap();

    if confirmation {
        println!("Continuing application...");
        c = true
    } else {
        println!("{} application.", Style::new().red().apply_to("Exiting"));
        c = false
    }

    c

}