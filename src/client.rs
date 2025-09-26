use std::{fs::{create_dir, read_to_string, write, DirEntry, ReadDir}, path::Path, thread::sleep, time::Duration};
use serde::{Deserialize, Serialize};

use crate::{cli::{self, clr, input, MenuOption}, main};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub name: String,
    pub description: String,
}

impl Client {
    
    pub fn new(name: String, description: String) -> Self {
        Client{
            name : name,
            description: description,
        }
    }

    pub fn get() -> Vec<Client> {
        let client_path = Path::new("clients");

        if ! client_path.is_dir() {
            create_dir(client_path);
        }

        let path_iter: ReadDir = Path::read_dir(&client_path).expect("No clients dir found.");
        let mut clients: Vec<Client> = Vec::new();

        for (n, dir_entry) in path_iter.enumerate() {
            let entry: DirEntry = dir_entry.unwrap();
            let entry_path = entry.path();
            let file_contents: String = read_to_string(&entry_path).unwrap();
            let client: Client = serde_json::from_str(&file_contents).unwrap();
            clients.push(client);
        }

        clients
    }

    pub fn save(&self){
        let clients_dir = Path::new("clients");
        
        if ! clients_dir.is_dir() {
            let __ = create_dir(clients_dir);
        }

        let filename: String = format!("{}.json", self.name);
        let filename_dir: String = format!("clients/{}", filename);
        let stringified_self: String = serde_json::to_string_pretty(&self).unwrap();
        let _ = write(&filename_dir, stringified_self);
    }

    pub fn to_vec(&self) -> Vec<String>{
        vec![
            self.name.to_string(),
            self.description.to_string()
        ]
    }

}

pub fn dialog(){
    cli::clr();
    let clients:Vec<Client> = Client::get();
    let mut clients_vec:Vec<Vec<String>> = Vec::new();
    clients.iter().for_each(|c: &Client| clients_vec.push(c.to_vec()));
    
    println!("[ My Clients ] ");
    cli::table(
        Some(vec![
            String::from("Name"),
            String::from("Description"),
        ]),
        &clients_vec,
    );

    cli::menu(None,vec![
        MenuOption::new("New Client", Some(create_dialog)),
        MenuOption::new("Delete Client", None),
        MenuOption::new("Exit", Some(main)),
    ]);
}

pub fn create_dialog(){
    clr();
    let name:String = input("Client Name : ", "Invalid Input.");
    let description:String = input("Client Description : ", "Invalid Input.");
    Client::new(name, description).save();
    println!("Client successfully created.");
    sleep(Duration::from_secs(2));
    main();
}

pub fn props() -> Vec<String> {
    vec![
        String::from("Name"),
        String::from("Description"),
    ]
}

