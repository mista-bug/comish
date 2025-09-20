use crate::{cli::{self, MenuOption}, main, medium};
use serde::{Deserialize, Serialize};
use std::{
    clone,
    fmt::format,
    fs::{self, create_dir},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Medium {
    pub name: String,
    pub price_per_square: f64,
}

impl Medium {
    pub fn new(name: String, price: f64) -> Self {
        Medium {
            name: name,
            price_per_square: price,
        }
    }

    pub fn get() -> Vec<Medium> {
        let dir = dir();
        let path_iter = Path::read_dir(&dir).expect("No mediums found.");
        let mut mediums: Vec<Medium> = Vec::new();
        for (n, dir_entry) in path_iter.enumerate() {
            let file = dir_entry.unwrap();
            let file_path = file.path();
            let file_contents: String = fs::read_to_string(&file_path).unwrap();
            let medium: Medium = serde_json::from_str(&file_contents).unwrap();
            mediums.push(medium);
        }
        mediums
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![self.name.to_string(), self.price_per_square.to_string()]
    }
}

pub fn dialog(){
    cli::clr();
    let mediums:Vec<Medium> = Medium::get();
    let mut mediums_vec:Vec<Vec<String>> = Vec::new();
    mediums
        .iter()
        .for_each(|m: &Medium| mediums_vec.push(m.to_vec()));
    
    println!("[ My Medium ] ");
    cli::table(
        Some(vec![
            String::from("Medium"),
            String::from("Price Per Sq"),
        ]),
        &mediums_vec,
    );

    cli::menu(None,vec![
        MenuOption::new("Create Medium", Some(create)),
        MenuOption::new("Delete Medium", None),
        MenuOption::new("Exit", Some(main)),
    ]);

}

pub fn create() {
    cli::clr();
    let _ = dir();
    let medium_name: String = cli::input("Medium Name : ", "Invalid Input");
    let price: f64 = cli::input("Price Per Square : ", "Invalid Input")
        .parse()
        .unwrap();

    let medium: Medium = Medium {
        name: medium_name,
        price_per_square: price,
    };

    let file_name: String = format!("{:?}.json", medium.name);
    let file_dir: String = format!("mediums/{file_name}");
    let medium_str: String = serde_json::to_string_pretty(&medium).unwrap();
    let _ = fs::write(&file_dir, &medium_str);
}

pub fn get() -> Vec<Medium> {
    let dir = dir();
    let path_iter = Path::read_dir(&dir).expect("No mediums found.");
    let mut mediums: Vec<Medium> = Vec::new();
    for (n, dir_entry) in path_iter.enumerate() {
        let file = dir_entry.unwrap();
        let file_path = file.path();
        let file_contents: String = fs::read_to_string(&file_path).unwrap();
        let medium: Medium = serde_json::from_str(&file_contents).unwrap();
        mediums.push(medium);
    }
    mediums
}

fn dir() -> PathBuf {
    let medium_path = Path::new("mediums");

    if !medium_path.is_dir() {
        create_dir(medium_path);
    }

    let cloned_path = medium_path.to_owned();
    cloned_path
}
