mod cli;
mod medium;
mod preset;
mod transaction;
use cli_table::{Cell, CellStruct, Style, Table, TableStruct, format::Border, print_stdout};
use serde::{
    Deserialize, Serialize,
    de::{self, value::Error},
};
use serde_json::{Value, to_string};
use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, ReadDir, create_dir},
    path::{Path, PathBuf},
    string, vec,
};

use crate::cli::{clr, MenuOption};

fn main() {
    cli::clr();
    cli::menu(Some(String::from("Welcome to Comish!")), vec![
        MenuOption::new("New Transaction", Some(transaction::new)),
        MenuOption::new("My Presets", Some(preset::dialog)),
        MenuOption::new("My Mediums", Some(medium::dialog)),
    ]);
}
