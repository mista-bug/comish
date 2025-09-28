use cli_table::{Cell, CellStruct, Table, TableStruct, print_stdout};
use std::io::{Write, stdin, stdout};

pub struct MenuOption{
    pub name: String,
    pub action: Option<fn()>
}

impl MenuOption {
    pub fn new(name:&str, action: Option<fn()>) -> Self {
        MenuOption { name: name.to_string() , action: action }
    }
}

pub fn input(input: &str, err: &str) -> String {
    let mut output: String = String::new();
    print!("{}", input);
    stdout().flush().unwrap();

    stdin().read_line(&mut output).expect(&err);

    return output.trim().to_string();
}

pub fn clr() -> () {
    print!("\x1B[2J");
}

pub fn table(header: Option<Vec<String>>, contents: &Vec<Vec<String>>) -> () {
    let mut table_data: Vec<Vec<CellStruct>> = Vec::new();

    let mut column_collection: Vec<CellStruct> = Vec::new();
    if let Some(h) = header {
        for column in h {
            column_collection.push(column.cell());
        }
    }

    for content in contents {
        let mut row_collection: Vec<CellStruct> = Vec::new();
        for row in content {
            row_collection.push(row.cell());
        }
        table_data.push(row_collection);
    }

    let mut table: TableStruct = table_data.table();
    if !column_collection.is_empty() {
        table = table.title(column_collection);
    }

    assert!(print_stdout(table).is_ok());
}

pub fn select_from_vec_struct<T>(message: String, choices: &Vec<T>) -> Option<&T> {
    let input_id: usize = input(&message, "Invalid Choice")
        .parse()
        .unwrap();

    choices.get(input_id)
}

pub fn menu(title: Option<String>, options: Vec<MenuOption>) -> () {
    if let Some(t) = title {
        println!("[ {t} ]");
    }

    for (n,option) in options.iter().enumerate() {
        println!(" {} ) {}", (n + 1).to_string() , option.name);
    }
    
    let mut input:usize = input("[ Enter ] : ", "Invalid Input.")
        .parse()
        .unwrap();

    input = input - 1;
    if let Some(output) = options.get(input){
        (output.action.unwrap())();
    } else {
        println!("Invalid Choice!");
    }
}

pub fn yn() -> bool {
    let input:char = input("Proceed ? [y/n]", "Invalid Input.")
    .parse()
    .unwrap();

    if input == 'y' || input == 'Y' {
        true
    } else if input == 'n' || input == 'N' {
        false
    } else {
        panic!()
    }
}

