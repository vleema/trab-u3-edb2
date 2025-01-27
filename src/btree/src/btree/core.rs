#[derive(Debug)]
pub enum BTree<T: Ord, const D: usize> {
    Leaf {
        keys: Vec<T>,
    },
    Node {
        keys: Vec<T>,
        childs: Vec<BTree<T, D>>,
    },
}

use std::mem;
use BTree::Leaf;
use BTree::Node;

#[allow(dead_code)]
impl<T: Ord, const D: usize> BTree<T, D> {
    pub fn new() -> Self {
        Leaf { keys: vec![] }
    }

    pub fn fetch(&self, value: &T) -> Option<&T> {
        match self {
            Leaf { keys } => keys
                .binary_search(value)
                .map_or_else(|_| None, |pos| Some(&keys[pos])),
            Node { keys, childs } => keys
                .binary_search(value)
                .map_or_else(|child| childs[child].fetch(value), |pos| Some(&keys[pos])),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.fetch(value).is_some()
    }

    pub fn insert(&mut self, value: T) {
        match self {
            Leaf { keys } => {
                keys.binary_search(&value)
                    .map_or_else(|insert_pos| keys.insert(insert_pos, value), |_| ());
                if keys.len() > 2 * D {
                    self.split_root_leaf();
                }
            }
            Node { keys, childs } => {
                if let Some((Some((middle_element, right_tree)), child_index)) =
                    keys.binary_search(&value).map_or_else(
                        |child_index| Some((childs[child_index].ins(value), child_index)),
                        |_| None,
                    )
                {
                    keys.insert(child_index, middle_element);
                    childs.insert(child_index + 1, right_tree);
                    if keys.len() > 2 * D {
                        self.split_root_node();
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, value: &T) -> Option<T> {
        match self {
            Leaf { keys } => keys
                .binary_search(value)
                .map_or(None, |i| keys.remove(i).into()),
            Node { keys, childs } => match keys.binary_search(value) {
                Ok(pos) => {
                    if keys.len() - 1 < D {
                        let sibling = get_sibling(childs, pos)
                            .expect("Unexpected Error: Internal node has no childs");
                        let sibling_ptr = &raw mut childs[sibling];
                        unsafe {
                            let sibling_key = {
                                if sibling > pos {
                                    (*sibling_ptr)
                                        .keys()
                                        .first()
                                        .expect("Unexpected Error: Empty sibling!")
                                } else {
                                    (*sibling_ptr)
                                        .keys()
                                        .last()
                                        .expect("Unexpected Error: Empty sibling!")
                                }
                            };
                            let mut removed_key = (*sibling_ptr)
                                .rem(sibling_key)
                                .expect("Unexpected Error while removing sibling key");
                            mem::swap(&mut keys[pos], &mut removed_key);
                            if childs[sibling].has_underflow() {
                                self.handle_underflow(sibling);
                            }
                            return Some(removed_key);
                        }
                    }
                    Some(keys.remove(pos))
                }
                Err(child) => {
                    let result = childs[child].rem(value);
                    if childs[child].keys().len() < D {
                        let sibling = get_sibling(childs, child)
                            .unwrap_or_else(|| panic!("Unexpected Error: subtree has no sibling"));
                        if childs[sibling].will_underflow() {
                            self.merge_siblings(child, sibling);
                        } else {
                            self.rebalance_with_sibling(child, sibling);
                        }
                    }
                    if self.childs().unwrap().len() == 1 {
                        let only_child = self.childs_mut().unwrap().pop().unwrap();
                        *self = only_child
                    }
                    result
                }
            },
        }
    }

    pub fn in_order_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        match self {
            Leaf { keys } => {
                result.extend(keys);
                result
            }
            Node { keys, childs } => {
                for i in 0..keys.len() {
                    result.extend(childs[i].in_order_traversal());
                    result.push(&keys[i]);
                }
                result.extend(childs[childs.len() - 1].in_order_traversal());
                result
            }
        }
    }

    pub fn keys(&self) -> &Vec<T> {
        match self {
            Leaf { keys } | Node { keys, .. } => keys,
        }
    }

    pub fn keys_mut(&mut self) -> &mut Vec<T> {
        match self {
            Leaf { keys } | Node { keys, .. } => keys,
        }
    }

    pub fn childs(&self) -> Option<&Vec<BTree<T, D>>> {
        if let Node { childs, .. } = self {
            return Some(childs);
        }
        None
    }

    pub fn childs_mut(&mut self) -> Option<&mut Vec<BTree<T, D>>> {
        if let Node { childs, .. } = self {
            return Some(childs);
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.keys().is_empty()
    }

    pub fn min(&self) -> Option<&T> {
        match self {
            Leaf { keys } => keys.first(),
            Node { childs, .. } => childs.first()?.min(),
        }
    }

    pub fn max(&self) -> Option<&T> {
        match self {
            Leaf { keys } => keys.last(),
            Node { childs, .. } => childs.last()?.max(),
        }
    }

    fn ins(&mut self, value: T) -> Option<(T, BTree<T, D>)> {
        match self {
            Leaf { keys } => {
                keys.binary_search(&value)
                    .map_or_else(|insert_pos| keys.insert(insert_pos, value), |_| ());
                if keys.len() > 2 * D {
                    return self.split_leaf();
                }
                None
            }
            Node { keys, childs } => {
                if let Some((Some((middle_element, right_tree)), child_index)) =
                    keys.binary_search(&value).map_or_else(
                        |child_index| Some((childs[child_index].ins(value), child_index)),
                        |_| None,
                    )
                {
                    keys.insert(child_index, middle_element);
                    childs.insert(child_index + 1, right_tree);
                    if keys.len() > 2 * D {
                        return self.split_node();
                    }
                }
                None
            }
        }
    }

    fn split_leaf(&mut self) -> Option<(T, BTree<T, D>)> {
        if let Leaf { keys } = self {
            if keys.is_empty() {
                return None;
            }
            let (first_half, middle_element, second_half) = split_keys(keys);
            let right_tree: BTree<T, D> = Leaf { keys: second_half };
            *keys = first_half;
            return Some((middle_element.unwrap(), right_tree));
        }
        None
    }

    fn split_node(&mut self) -> Option<(T, BTree<T, D>)> {
        if let Node { keys, childs } = self {
            if keys.is_empty() {
                return None;
            }
            let (first_half, middle_element, second_half) = split_keys(keys);
            let (left_childs, right_childs) = split_childs(childs);
            *self = Node {
                keys: first_half,
                childs: left_childs,
            };
            let right_tree = Node {
                keys: second_half,
                childs: right_childs,
            };
            return Some((middle_element.unwrap(), right_tree));
        }
        None
    }

    fn split_root_leaf(&mut self) {
        if let Leaf { keys } = self {
            if keys.is_empty() {
                return;
            }
            let (first_half, middle_element, second_half) = split_keys(keys);
            let left_tree: BTree<T, D> = Leaf { keys: first_half };
            let right_tree: BTree<T, D> = Leaf { keys: second_half };
            *self = Node {
                keys: vec![middle_element.unwrap()],
                childs: vec![left_tree, right_tree],
            };
        }
    }

    fn split_root_node(&mut self) {
        if let Node { keys, childs } = self {
            if keys.is_empty() {
                return;
            }
            let (first_half, middle_element, second_half) = split_keys(keys);
            let (left_childs, right_childs) = split_childs(childs);
            let left_tree = Node {
                keys: first_half,
                childs: left_childs,
            };
            let right_tree = Node {
                keys: second_half,
                childs: right_childs,
            };
            *self = Node {
                keys: vec![middle_element.unwrap()],
                childs: vec![left_tree, right_tree],
            };
        }
    }

    fn rem(&mut self, value: &T) -> Option<T> {
        match self {
            Leaf { keys } => keys
                .binary_search(value)
                .map_or(None, |i| keys.remove(i).into()),
            Node { keys, childs } => match keys.binary_search(value) {
                Ok(pos) => {
                    if keys.len() - 1 < D {
                        let sibling = get_sibling(childs, pos)
                            .expect("Unexpected Error: Internal node has no childs");
                        let sibling_ptr = &raw mut childs[sibling];
                        unsafe {
                            let sibling_key = {
                                if sibling > pos {
                                    (*sibling_ptr)
                                        .keys()
                                        .first()
                                        .expect("Unexpected Error: Empty sibling!")
                                } else {
                                    (*sibling_ptr)
                                        .keys()
                                        .last()
                                        .expect("Unexpected Error: Empty sibling!")
                                }
                            };
                            let mut removed_key = (*sibling_ptr)
                                .rem(sibling_key)
                                .expect("Unexpected Error while removing sibling key");
                            mem::swap(&mut keys[pos], &mut removed_key);
                            if childs[sibling].has_underflow() {
                                self.handle_underflow(sibling);
                            }
                            return Some(removed_key);
                        }
                    }
                    Some(keys.remove(pos))
                }
                Err(child) => {
                    let result = childs[child].rem(value);
                    if childs[child].has_underflow() {
                        self.handle_underflow(child);
                    }
                    result
                }
            },
        }
    }

    fn handle_underflow(&mut self, child: usize) {
        let childs = self.childs_mut().unwrap();
        let sibling = get_sibling(childs, child).expect("Unexpected Error: subtree has no sibling");
        if childs[sibling].will_underflow() {
            self.merge_siblings(child, sibling);
        } else {
            self.rebalance_with_sibling(child, sibling);
        }
    }

    fn merge_siblings(&mut self, child: usize, sibling: usize) {
        match self {
            Leaf { .. } => (),
            Node { keys, childs } if sibling != child => {
                let (merge_target, merge_source, parent_index) = if sibling > child {
                    (child, sibling, child)
                } else {
                    (sibling, child, sibling)
                };
                let parent = keys.remove(parent_index);
                childs[merge_target].keys_mut().push(parent);

                let mut source_tree = childs.remove(merge_source);
                childs[merge_target]
                    .keys_mut()
                    .append(source_tree.keys_mut());

                let target_children = childs[merge_target].childs_mut();
                let source_children = source_tree.childs_mut();
                match (target_children, source_children) {
                    (Some(target_children), Some(source_children)) => {
                        target_children.append(source_children);
                    }
                    (None, None) => {}
                    _ => {
                        panic!("Unexpected Error: Cannot merge a leaf node with internal node")
                    }
                }
            }
            Node { .. } => {
                panic!("Unexpected Error: Cannot merge node with itself");
            }
        }
    }

    fn rebalance_with_sibling(&mut self, child: usize, sibling: usize) {
        match self {
            Leaf { .. } => {
                panic!("Unexpected Error: Leaf node has no childs");
            }
            Node { keys, childs } => {
                if child >= childs.len() || sibling >= childs.len() {
                    panic!("Unexpected Error: Out of bounds child and sibling index")
                }
                let is_left_sibling = sibling <= child;
                let parent = if is_left_sibling {
                    &mut keys[sibling]
                } else {
                    &mut keys[child]
                };

                if is_left_sibling {
                    let mut moved_key = childs[sibling].keys_mut().pop().unwrap_or_else(|| {
                        panic!("Unexpected Error: Sibling has no spare elements at this point")
                    });
                    mem::swap(parent, &mut moved_key);
                    childs[child].keys_mut().insert(0, moved_key);
                } else {
                    let mut moved_key = childs[sibling].keys_mut().remove(0);
                    mem::swap(parent, &mut moved_key);
                    childs[child].keys_mut().push(moved_key);
                }
            }
        }
    }

    fn has_underflow(&self) -> bool {
        self.keys().len() < D
    }

    fn will_underflow(&self) -> bool {
        self.keys().len() - 1 < D
    }
}

impl<T: Ord, const D: usize> Default for BTree<T, D> {
    fn default() -> Self {
        Self::new()
    }
}

fn get_sibling<T: Ord, const D: usize>(childs: &[BTree<T, D>], child: usize) -> Option<usize> {
    let left_sibling = if child > 0 { Some(child - 1) } else { None };
    let right_sibling = if child + 1 < childs.len() {
        Some(child + 1)
    } else {
        None
    };

    match (left_sibling, right_sibling) {
        (Some(left), Some(right)) => {
            if childs[right].keys().len() >= childs[left].keys().len() {
                return Some(right);
            };
            Some(left)
        }
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

fn split_childs<T: Ord, const D: usize>(
    childs: &mut Vec<BTree<T, D>>,
) -> (Vec<BTree<T, D>>, Vec<BTree<T, D>>) {
    let len = childs.len();
    if len == 0 {
        return (vec![], vec![]);
    }
    let mid = len / 2;
    let right_childs: Vec<BTree<T, D>> = childs.split_off(mid);
    (mem::take(childs), right_childs)
}

fn split_keys<T>(vec: &mut Vec<T>) -> (Vec<T>, Option<T>, Vec<T>) {
    let len = vec.len();
    if len == 0 {
        return (vec![], None, vec![]);
    }
    let mid = len / 2;
    let second_half = vec.split_off(mid + 1);
    let middle_element = vec.pop();
    (mem::take(vec), middle_element, second_half)
}
