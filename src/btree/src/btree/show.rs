use super::core::BTree;
use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
};

impl<T: Ord + Display, const D: usize> BTree<T, D> {
    pub fn generate_graph(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "digraph BTree{{")?;
        writeln!(file, "node [shape=record];")?;

        let mut next_id = 1;
        self.generate_dot(&mut file, 0, &mut next_id)?;

        writeln!(file, "}}")?;
        println!("File {} generated for Btree visualization", filename);
        Ok(())
    }

    fn generate_dot(&self, out: &mut impl Write, id: usize, next_id: &mut usize) -> io::Result<()> {
        write!(out, "Node{} [label=\"", id)?;
        for key in self.keys() {
            write!(out, "{} ", key)?;
        }
        writeln!(out, "\"];")?;

        if let Some(childs) = self.childs() {
            for child in childs {
                let child_id = *next_id;
                *next_id += 1;
                writeln!(out, "Node{} -> Node{};", id, child_id)?;
                child.generate_dot(out, child_id, next_id)?;
            }
        }
        Ok(())
    }
}
