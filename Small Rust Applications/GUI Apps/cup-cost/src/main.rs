slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let mut num_cups: f32 = 0.0;
    let mut num_straws: f32 = 0.0;
    let mut num_energy_drinks: f32 = 0.0;
    let mut num_srv_syrup: f32 = 0.0;
    let mut num_srv_mixer: f32 = 0.0;

    let mut cups_cost: f32 = 0.00;
    let mut straws_cost: f32 = 0.00;
    let mut energy_drinks_cost: f32 = 0.00;
    let mut syrup_cost: f32 = 0.00;
    let mut mixer_cost: f32 = 0.00;

    let ui = CupCost::new()?;
    let ui_handle = ui.as_weak();

    ui.on_calculate_cost(   move |
                            str_cups, str_cups_price, 
                            str_straws, str_straws_price, 
                            str_energy, str_energy_price, 
                            str_syrup, str_syrup_price,
                            str_mixer, str_mixer_price
                            | {
        let ui: CupCost = ui_handle.unwrap();

        num_cups = str_cups.trim().parse().unwrap();
        cups_cost = str_cups_price.trim().parse().unwrap();
        num_straws = str_straws.trim().parse().unwrap();
        straws_cost = str_straws_price.trim().parse().unwrap();
        num_energy_drinks = str_energy.trim().parse().unwrap();
        energy_drinks_cost = str_energy_price.trim().parse().unwrap();
        num_srv_syrup = str_syrup.trim().parse().unwrap();
        syrup_cost = str_syrup_price.trim().parse().unwrap();
        num_srv_mixer = str_mixer.trim().parse().unwrap();
        mixer_cost = str_mixer_price.trim().parse().unwrap();

        let cups_total = cups_cost / num_cups;
        let straws_total = straws_cost / num_straws;
        let edrink_total = energy_drinks_cost / num_energy_drinks;
        let syrup_total = syrup_cost / num_srv_syrup;
        let mixer_total = mixer_cost / num_srv_mixer;
        let total_cost = cups_total + straws_total + edrink_total + syrup_total + mixer_total;

        let result = format!("  Price per Cup & Lid: ${:.2}\n
                                        Price per Straw: ${:.2}\n
                                        Price per Energy Drink: ${:.2}\n
                                        Price per Syrup Serving: ${:.2}\n
                                        Price Per Mixer Serving: ${:.2}\n
                                        Total Cost: ${:.2}",
                                        cups_total, 
                                        straws_total,
                                        edrink_total,
                                        syrup_total,
                                        mixer_total,
                                        total_cost);
        
        ui.set_results(result.into());
    });

    ui.run()
}
