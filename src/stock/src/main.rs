mod stock;

use std::io;

use btree::btree::core::BTree;
use stock::Data;

fn main() -> io::Result<()> {
    let arroz = Data::new(1, "arroz".to_string(), 40);
    let feijao = Data::new(2, "feijao".to_string(), 53);
    let macarrao = Data::new(3, "macarrao".to_string(), 60);
    let alface = Data::new(4, "alface".to_string(), 30);
    let farofa = Data::new(5, "farofa".to_string(), 70);
    let carne = Data::new(6, "carne".to_string(), 2);
    let mut tree: BTree<Data, 2> = BTree::new();

    tree.insert(arroz);
    tree.insert(feijao);
    tree.insert(macarrao);
    tree.insert(alface);
    tree.insert(farofa);
    tree.insert(carne);

    tree.generate_graph("out/dot/datas.dot")?;
    Ok(())
}
