mod btree;

use std::{io, process::Command};

use btree::core::BTree;

fn main() -> io::Result<()> {
    {
        let mut tree: BTree<i32, 2> = BTree::Leaf {
            keys: vec![1, 2, 3, 4, 5],
        };
        tree.insert(6);
        tree.generate_graph("out/dot/small_btree.dot")?;
        Command::new("dot")
            .args([
                "-Tpng",
                "out/dot/small_btree.dot",
                "-o",
                "out/png/small_btree.png",
            ])
            .spawn()
            .expect("dot failed to execute");
    }
    {
        let vec = vec![
            11, 43, 67, 89, 2, 5, 7, 17, 29, 31, 37, 59, 61, 71, 79, 83, 97, 107, 110, 111, 60, 62,
            63,
        ];
        let mut tree: BTree<i32, 2> = BTree::Leaf { keys: vec![] };
        for elem in vec {
            tree.insert(elem);
        }
        tree.generate_graph("out/dot/medium_btree.dot")?;
        Command::new("dot")
            .args([
                "-Tpng",
                "out/dot/medium_btree.dot",
                "-o",
                "out/png/medimum_btree.png",
            ])
            .spawn()
            .expect("dot failed to execute");
    }
    {
        let vec = vec![
            11, 43, 67, 89, 2, 5, 7, 17, 29, 31, 37, 59, 61, 71, 79, 83, 97, 107, 110, 111, 60, 62,
            63, 15, 28, 45, 66, 73, 81, 92, 104, 113, 124, 132, 146, 157, 169, 182, 193, 205, 218,
            230, 241, 252, 264, 275, 287, 298, 310, 322, 333, 345, 357, 368, 380, 391, 403, 414,
            426, 438, 449, 461, 473, 484, 496, 507, 519, 530, 542, 554, 565, 577, 589, 600, 612,
            623, 635, 647, 658, 670, 682, 693, 705, 717, 728, 740, 752, 763, 775, 787, 798, 810,
            822, 833, 845, 857, 868, 880, 892, 903, 915, 927, 938, 950, 962, 973, 985, 997,
        ];

        let mut tree: BTree<i32, 2> = BTree::Leaf { keys: vec![] };
        for elem in vec {
            tree.insert(elem);
        }
        tree.generate_graph("out/dot/large_btree.dot")?;
        Command::new("dot")
            .args([
                "-Tpng",
                "out/dot/large_btree.dot",
                "-o",
                "out/png/large_btree.png",
            ])
            .spawn()
            .expect("dot failed to execute");
    }
    Ok(())
}
