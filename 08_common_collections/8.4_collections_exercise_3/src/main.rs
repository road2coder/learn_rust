use std::{collections::HashMap, io};

fn main() {
    const HELP_MSG: &str = "\n\
        Hello administrator. What do you want to do?\n\
        Here are available commands:\n\t\
        - 'add <Name> to <Department>'\n\t\
        - 'list <Department>(optional)'\n\t\
        - 'exit'";
    let mut company = HashMap::new();
    let mut s = String::new();
    println!("{HELP_MSG}");
    loop {
        io::stdin().read_line(&mut s).expect("Failed to read line!");
        let words: Vec<&str> = s.split_whitespace().collect();
        match words.as_slice() {
            ["add", name, "to", dept] => company
                .entry(dept.to_string())
                .or_insert(Vec::new())
                .push(name.to_string()),
            ["list", dept] => print_dept(&company, dept),
            ["list"] => {
                if company.is_empty() {
                    println!("Company is empty!");
                } else {
                    let mut keys: Vec<&String> = company.keys().collect();
                    keys.sort();
                    for dept in keys {
                        println!("{dept}:");
                        print_dept(&company, dept);
                    }
                }
            }
            ["exit", ..] => break,
            _ => println!("Unknown command"),
        }
        s.clear();
    }
}

fn print_dept(company: &HashMap<String, Vec<String>>, dept: &str) {
    if let Some(names) = company.get(dept) {
        let mut names: Vec<&String> = names.iter().collect();
        names.sort();
        for name in names {
            println!("\t[{name}]");
        }
    } else {
        println!("Unknown department {dept}");
    }
}
