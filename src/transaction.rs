use cli_table::Cell;
use serde::{Deserialize, Serialize};

use crate::{cli::{self, clr, input, menu, select_from_vec_struct, table, yn, MenuOption}, client::{self, Client}, main, medium::Medium, preset::{self, Preset}, transaction};
use std::{
    fs::{self, create_dir, read_dir, write, DirEntry, ReadDir}, path::Path, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}
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
        
        let now: SystemTime = SystemTime::now();
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

    pub fn get() -> Vec<Transaction> {
        let path = Path::new("transactions");

        if ! path.is_dir() {
            let _ = create_dir(path);
        }

        let path_iter: ReadDir = Path::read_dir(&path).expect("No transactions dir found.");
        let mut transactions: Vec<Transaction> = Vec::new();

        for (n, dir_entry) in path_iter.enumerate() {
            let entry: DirEntry = dir_entry.unwrap();
            let entry_path = entry.path();
            let file_contents: String = fs::read_to_string(&entry_path).unwrap();
            let transaction: Transaction = serde_json::from_str(&file_contents).unwrap();
            transactions.push(transaction);
        }
        transactions
    }

    pub fn save(&self){
        let transactions_dir = Path::new("transactions");
        
        if ! transactions_dir.is_dir() {
            let __ = create_dir(transactions_dir);
        }

        let filename: String = format!("{}.json", self.date);
        let filename_dir: String = format!("transactions/{}", filename);
        let stringified_self: String = serde_json::to_string_pretty(&self).unwrap();
        let _ = write(&filename_dir, stringified_self);
    }

    pub fn to_vec(&self) -> Vec<String>{
        vec![
            self.preset.name.to_string(),
            self.client.name.to_string(),
            self.total_price.to_string(),
            self.date.to_string(),
        ]
    }

}

pub fn dialog(){
    clr();
    let transactions = Transaction::get();
    let mut contents:Vec<Vec<String>> = Vec::new();
    transactions.iter().for_each(|t| contents.push(t.to_vec()));
    table(Some(props()),  &contents);
    menu(Some(String::from("Transactions")), vec![
        MenuOption::new("New", Some(new_dialog)),
        MenuOption::new("Exit", Some(main)),
    ]);
} 

pub fn new_dialog() {
    
    //select client
    clr();
    let clients = Client::get();
    let clients_strings: Vec<Vec<String>> = clients
    .iter()
    .enumerate()
    .map(|(n, client)| {
        let mut c_vec= vec![n.to_string()];
        c_vec.append(&mut client.to_vec());
        c_vec
    }).collect();
    let mut cols = vec![String::from("#")];
    cols.append(&mut client::props());
    table(Some(cols), &clients_strings);
    let client = select_from_vec_struct(String::from("Client: "), &clients).unwrap();

    //select preset
    clr();
    let presets = Preset::get();
    let presets_strings: Vec<Vec<String>> = presets
    .iter()
    .enumerate()
    .map(|(n, preset)| {
        let mut p_vec= vec![n.to_string()];
        p_vec.append(&mut preset.to_vec());
        p_vec
    }).collect();
    let mut cols_presets = vec![String::from("#")];
    cols_presets.append(&mut preset::props());
    table(Some(cols_presets),&presets_strings);
    let preset = select_from_vec_struct(String::from("Preset: "), &presets).unwrap();
    let transaction:Transaction = Transaction::new(preset.clone(), client.clone());

    //show
    clr();
    table(None, &vec![
        vec!["Preset".to_string(), preset.name.to_string()],
        vec!["Client".to_string(), client.name.to_string()],
        vec!["Price".to_string(), transaction.total_price.to_string()]
    ]);
    
    let yn = yn();
    if yn {
        transaction.save();
        println!("Transaction Complete.");
        sleep(Duration::from_secs(1));
        clr();
        main();
    } else {
        println!("Transaction Cancelled.");
        sleep(Duration::from_secs(1));
        clr();
        main();
    }

}

fn total_price(preset: &Preset) -> f64 {
    let base_price:f64 = preset.base_price;
    let price_per_square:f64 = preset.medium.price_per_square;
    let a:f64 = (preset.canvas_width * preset.canvas_height) as f64;
    base_price + (a * price_per_square)
}

pub fn props() -> Vec<String> {
    vec![
        String::from("Preset"),
        String::from("Client"),
        String::from("Price"),
        String::from("Date"),
    ]
}
