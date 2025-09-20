use crate::{cli, preset::Preset};
use std::{
    fs::{self, DirEntry, ReadDir, create_dir},
    path::Path,
};

pub fn new() {
    cli::clr();
    let preset_path = Path::new("presets");

    if !preset_path.is_dir() {
        create_dir(preset_path);
    }

    let path_iter: ReadDir = Path::read_dir(&preset_path).expect("No preset dir found.");

    //show existing presets
    let mut presets_collection: Vec<Vec<String>> = Vec::new();
    for (n, entry) in path_iter.enumerate() {
        let entry: DirEntry = entry.unwrap();
        let entry_path = entry.path();
        let file_name: &str = entry_path.file_stem().unwrap().to_str().unwrap();

        let file_contents: String = fs::read_to_string(&entry_path).unwrap();
        let preset: Preset = serde_json::from_str(&file_contents).unwrap();
        let entry_n: String = n.to_string();

        presets_collection.push(vec![
            entry_n,
            file_name.to_string(),
            preset.canvas_width.to_string(),
            preset.canvas_height.to_string(),
            serde_json::to_string_pretty(&preset.medium).unwrap(),
            preset.base_price.to_string(),
        ]);
    }
    let preset_header = vec![
        String::from("#"),
        String::from("Preset Name"),
        String::from("Width"),
        String::from("Height"),
        String::from("Medium"),
        String::from("Base Price"),
    ];

    cli::table(Some(preset_header), &presets_collection);

    let item: Vec<String> =
        cli::select_from_vec_by_id(String::from("Select preset : "), &presets_collection).unwrap();
}
