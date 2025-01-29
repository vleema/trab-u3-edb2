mod stock;

use btree::btree::core::BTree;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use stock::Data;

fn read_file(path: &str) -> io::Result<Vec<Data>> {
    let mut datas = Vec::new();

    let file = File::open(Path::new(path))?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some((id, name, stock)) = parse_line(&line) {
            datas.push(Data::new(id, name, stock));
        }
    }

    Ok(datas)
}

fn parse_line(line: &str) -> Option<(usize, String, usize)> {
    let content = line.trim_matches(|c| c == '{' || c == '}');
    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

    if parts.len() == 3 {
        if let (Ok(id), Some(name), Ok(stock)) = (
            parts[0].parse::<usize>(),
            parts.get(1).map(|s| s.trim_matches('"').to_string()),
            parts[2].parse::<usize>(),
        ) {
            return Some((id, name, stock));
        }
    }

    None
}

fn print_menu_options() {
    println!("Please choose an option:");
    println!(" 1 - Search item");
    println!(" 2 - Insert item");
    println!(" 3 - Delete item");
    println!(" 4 - Quit\n");
}

fn handle_menu(menu_option: usize, tree: &mut BTree<Data, 2>) {
    match menu_option {
        1 => {
            println!("ID:");
            let id_search = read_input::<usize>();
            if let Some(item) = tree.fetch(&Data::new(id_search, String::new(), 0)) {
                println!("Item found");
                println!("ID: {}, Name: {}, Stock: {}", item.id(), item.name(), item.stock());
            } else {
                println!("Item not found");
            }
        }
        2 => {
            println!("ID:");
            let id = read_input::<usize>();
            println!("Name:");
            let name = read_input::<String>();
            println!("Number of items in stock:");
            let stock = read_input::<usize>();
            tree.insert(Data::new(id, name, stock));
        }
        3 => {
            println!("ID:");
            let id_remove = read_input::<usize>();
            tree.remove(&Data::new(id_remove, String::new(), 0));
            println!("Item deleted.");
        }
        4 => {
            println!("Exiting the program.");
        }
        _ => {
            println!("Invalid option. Please try again.");
        }
    }
}

fn read_input<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().ok().expect("Invalid input")
}

fn main() -> io::Result<()> {
    let mut tree: BTree<Data, 2> = BTree::new();

    let file_path = "dadosB.txt";

    match read_file(file_path) {
        Ok(datas) => {
            for data in datas {
                tree.insert(data);
            }
        }
        Err(erro) => {
            eprintln!("Failed to read file: {}", erro);
        }
    }

    loop {
        print_menu_options();
        let menu_option = read_input::<usize>();
        if menu_option == 4 {
            break;
        }
        handle_menu(menu_option, &mut tree);
    }

    tree.generate_graph("out/dot/datas.dot")?;
    Ok(())
}
