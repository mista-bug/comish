mod cli;
mod medium;
mod preset;
mod transaction;
mod client;
use crate::cli::{MenuOption};

fn main() {
    cli::clr();
    cli::menu(Some(String::from("Welcome to Comish!")), vec![
        MenuOption::new("Transactions", Some(transaction::dialog)),
        MenuOption::new("My Clients", Some(client::dialog)),
        MenuOption::new("My Presets", Some(preset::dialog)),
        MenuOption::new("My Mediums", Some(medium::dialog)),
        MenuOption::new("Exit", None),
    ]);
}
