use btree::btree::core::*;

#[test]
fn test_new_tree_is_empty() {
    let tree: BTree<i32, 2> = BTree::new();
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_search() {
    let mut tree: BTree<i32, 2> = BTree::new();
    tree.insert(42);
    assert!(tree.contains(&42));
    assert!(!tree.contains(&99));
}

#[test]
fn test_multiple_inserts() {
    let mut tree: BTree<i32, 2> = BTree::new();
    for i in 0..100 {
        tree.insert(i);
    }

    for i in 0..100 {
        println!("testing: {}", i);
        assert!(tree.contains(&i));
    }
}

#[test]
fn test_remove() {
    let mut tree: BTree<i32, 2> = BTree::new();
    tree.insert(10);
    assert!(tree.contains(&10));

    tree.remove(&10);
    assert!(!tree.contains(&10));
}

#[test]
fn test_delete_non_existent() {
    let mut tree: BTree<i32, 2> = BTree::new();
    tree.remove(&99);
}

#[test]
fn test_in_order_traversal() {
    let mut tree: BTree<i32, 2> = BTree::new();
    tree.insert(3);
    tree.insert(1);
    tree.insert(4);
    tree.insert(2);

    assert_eq!(tree.in_order_traversal(), vec![&1, &2, &3, &4]);
}

#[test]
fn test_tree_reorganization() {
    let mut tree: BTree<i32, 2> = BTree::new();
    let nums = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];

    for n in &nums {
        tree.insert(*n);
    }

    // Remove middle elem
    tree.remove(&50);
    assert!(!tree.contains(&50));

    assert_eq!(
        tree.in_order_traversal(),
        vec![&10, &20, &30, &40, &60, &70, &80, &90]
    );
}

#[test]
fn test_min_and_max() {
    let mut tree: BTree<i32, 2> = BTree::new();
    let nums = vec![15, 8, 42, 4, 23, 16];

    for n in &nums {
        tree.insert(*n);
    }

    assert_eq!(tree.min(), Some(&4));
    assert_eq!(tree.max(), Some(&42));
}

#[test]
fn test_consecutive_insert_delete() {
    let mut tree: BTree<i32, 2> = BTree::new();

    for i in 0..100 {
        tree.insert(i);
        assert!(tree.contains(&i));

        tree.remove(&i);
        assert!(!tree.contains(&i));
    }

    assert!(tree.is_empty());
}
