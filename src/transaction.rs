use serde::{Deserialize, Serialize};

use crate::{cli::{self, input, select_from_vec_struct, table}, client::{self, Client}, medium::Medium, preset::{self, Preset}};
use std::{
    fs::{self, create_dir, read_dir, write, DirEntry, ReadDir},
    path::Path, time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub preset:Preset,
    pub client:Client,
    pub total_price: f64,
    pub date:u128,
}

impl Transaction {
    pub fn new(
        preset: Preset,
        client: Client,
    ) -> Self {
        
        let now = SystemTime::now();
        let sys_epoch = now.duration_since(UNIX_EPOCH).expect("Time error");
        let ms = sys_epoch.as_millis();
        let total_price = total_price(&preset);

        Transaction { 
            preset: preset,
            client: client,
            total_price: total_price,
            date: ms,
        }
    }

    pub fn save(&self){
        let transactions_dir = Path::new("transactions");
        
        if ! transactions_dir.is_dir() {
            let __ = create_dir(transactions_dir);
        }

        let file_count = read_dir(transactions_dir).iter().count();

        let filename: String = format!("{}.json", file_count);
        let filename_dir: String = format!("transactions/{}", filename);
        let stringified_self: String = serde_json::to_string_pretty(&self).unwrap();
        let _ = write(&filename_dir, stringified_self);
    }

    

    pub fn to_vec(&self) -> Vec<String>{
        vec![
            self.preset.name.to_string(),
            self.client.name.to_string(),
            self.date.to_string(),
        ]
    }

}

pub fn dialog() {
    //select client
    let clients = Client::get();
    let mut clients_strings: Vec<Vec<String>> = Vec::new();
    clients.iter().for_each(|c| clients_strings.push(c.to_vec()));
    table(Some(client::props()), &clients_strings);
    let client = select_from_vec_struct(String::from("Client: "), &clients).unwrap();

    //select preset
    let presets = Preset::get();
    let mut presets_strings = Vec::new();
    presets.iter().for_each(|p| presets_strings.push(p.to_vec()));
    table(Some(preset::props()),&presets_strings);
    let preset = select_from_vec_struct(String::from("Preset: "), &presets).unwrap();

    Transaction::new(preset.clone(), client.clone()).save();
}

fn total_price(preset: &Preset) -> f64 {
    let mut total_price:f64 = 0.00;
    let base_price:f64 = preset.base_price;
    let price_per_square:f64 = preset.medium.price_per_square;
    let w:u32 = preset.canvas_width;
    let h:u32 = preset.canvas_height;
    let a:f64 = (w * h) as f64;
    total_price = a * price_per_square;
    total_price
}
