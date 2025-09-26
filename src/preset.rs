use core::time;
use std::{
    fs::{self, create_dir, DirEntry, ReadDir},
    path::{self, Path}, thread::sleep,
};

use crate::{
    cli::{self, clr, menu, MenuOption}, main, medium::{self, Medium}, preset, transaction
};
use cli_table::{Cell, CellStruct, Style, TableStruct, print_stdout};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preset {
    pub name: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub medium: Medium,
    pub base_price: f64,
}

impl Preset {
    pub fn new(name: String, width: u32, height: u32, medium: Medium, base_price: f64) -> Self {
        Preset {
            name: name,
            canvas_width: width,
            canvas_height: height,
            medium: medium,
            base_price: base_price,
        }
    }

    pub fn get() -> Vec<Preset> {
        let preset_path = Path::new("presets");

        if !preset_path.is_dir() {
            create_dir(preset_path);
        }

        let path_iter: ReadDir = Path::read_dir(&preset_path).expect("No preset dir found.");
        let mut presets: Vec<Preset> = Vec::new();

        for (n, dir_entry) in path_iter.enumerate() {
            let entry: DirEntry = dir_entry.unwrap();
            let entry_path = entry.path();
            let file_contents: String = fs::read_to_string(&entry_path).unwrap();
            let preset: Preset = serde_json::from_str(&file_contents).unwrap();
            presets.push(preset);
        }
        presets
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.canvas_width.to_string(),
            self.canvas_height.to_string(),
            self.medium.name.to_string(),
            self.base_price.to_string(),
        ]
    }
}

pub fn dialog() {
    clr();
    let presets: Vec<Preset> = Preset::get();
    let mut presets_vec: Vec<Vec<String>> = Vec::new();
    presets
        .iter()
        .for_each(|p: &Preset| presets_vec.push(p.to_vec()));
    
    println!("[ My Presets ] ");
    cli::table(
        Some(vec![
            String::from("Preset Name"),
            String::from("W"),
            String::from("H"),
            String::from("Medium"),
            String::from("Base Price"),
        ]),
        &presets_vec,
    );

    cli::menu(None,vec![
        MenuOption::new("Create Preset", Some(new)),
        MenuOption::new("Delete Preset", None),
        MenuOption::new("Exit", Some(main)),
    ]);

}

pub fn new() {
    let preset: Preset = new_preset_menu();
    let dir_path = Path::new("presets");

    if !dir_path.is_dir() {
        let _ = fs::create_dir(&dir_path);
    }

    let filename: String = format!("{}.json", preset.name);
    let filename_dir: String = format!("presets/{}", filename);
    let preset_string: String = serde_json::to_string_pretty(&preset).unwrap();
    let _ = fs::write(&filename_dir, preset_string);
}

fn new_preset_menu() -> Preset {
    let name: String = cli::input("Preset Name : ", "Input error.");
    let base_price: f64 = cli::input("Base Price : ", "Input error.").parse().unwrap();

    let canvas_width: u32 = cli::input("Canvas Width : ", "Input error.")
        .parse()
        .unwrap();

    let canvas_height: u32 = cli::input("Canvas Height : ", "Input error.")
        .parse()
        .unwrap();

    let title: Vec<String> = vec![
        String::from("#"),
        String::from("Medium"),
        String::from("Price per square"),
    ];

    //display mediums
    let mediums: Vec<Medium> = medium::get();
    let mut medium_rows: Vec<Vec<String>> = Vec::new();
    for (n, medium) in mediums.iter().enumerate() {
        medium_rows.push(vec![
            n.to_string(),
            medium.name.clone(),
            medium.price_per_square.to_string(),
        ]);
    }

    cli::table(Some(title), &medium_rows);
    let medium:Medium =
        cli::select_from_vec_struct(String::from("Select Medium"), &medium::get())
        .unwrap()
        .clone();

    let preset: Preset = Preset {
        name: name,
        canvas_width: canvas_width,
        canvas_height: canvas_height,
        medium: medium,
        base_price: base_price,
    };
    println!("Preset {} Created.",preset.name);
    sleep(time::Duration::from_secs(2));
    return preset;
}

pub fn props() -> Vec<String>{
    vec![
        String::from("Name"),
        String::from("Canvas Width"),
        String::from("Canvas Height"),
        String::from("Medium"),
        String::from("Base Price"),
    ]
}
