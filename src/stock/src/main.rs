mod stock;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use btree::btree::core::BTree;
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

fn main() -> io::Result<()> {
    let mut tree: BTree<Data, 2> = BTree::new();

    let file_path = "dadosB.txt";

    match read_file(file_path) {
        Ok(dados) => {
            for data in dados {
                tree.insert(data);
            }
        }
        Err(erro) => {
            eprintln!("Erro ao ler o arquivo: {}", erro);
        }
    }

    tree.generate_graph("out/dot/datas.dot")?;
    Ok(())
}

