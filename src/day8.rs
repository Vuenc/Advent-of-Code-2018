use std::cell::Cell;

struct Node {
    childs: Vec<Node>,
    metadata_entries: Vec<i32>,
    value: Cell<Option<i32>>,
}

impl Node {
    pub fn new() -> Node {
        Node {childs: Vec::new(), metadata_entries: Vec::new(), value: Cell::new(None)}
    }

    pub fn metadata_sum_recursive(&self) -> i32 {
        self.metadata_entries.iter().sum::<i32>() +
            self.childs.iter().map(|child| child.metadata_sum_recursive()).sum::<i32>()
    }
}

fn parse_node(remaining_array: &[i32]) -> (Node, usize) {
    let (child_amount, metadata_amount) = (remaining_array[0], remaining_array[1] as usize);
    let mut current_root = Node::new();
    let mut next_child_index = 2;
    for _i in 0..child_amount {
        let (subtree, subtree_length) = parse_node(&remaining_array[next_child_index..]);
        current_root.childs.push(subtree);
        next_child_index += subtree_length;
    }
    for metadata_entry in &remaining_array[next_child_index..next_child_index + metadata_amount] {
        current_root.metadata_entries.push(*metadata_entry)
    }
    let length = next_child_index + metadata_amount;
    (current_root, length)
}

pub fn star1(line: &String) -> String {
    let tokens = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let (root_node, _) = parse_node(&tokens);

    root_node.metadata_sum_recursive().to_string()
}

fn calculate_node_value(node: &Node) -> i32 {
    if let Some(value) = node.value.get() {
        return value;
    }
    let value = if node.childs.len() == 0 {
        node.metadata_entries.iter().sum()
    } else {
        node.metadata_entries.iter()
            .map(|&entry| {
                let uentry = entry as usize;
                if uentry > 0 && uentry <= node.childs.len() {
                    calculate_node_value(&node.childs[uentry - 1])
                } else {
                    0
                }
            })
            .sum()
    };
    node.value.set(Some(value));
    value
}

pub fn star2(line: &String) -> String {
    let tokens = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let (root_node, _) = parse_node(&tokens);

    calculate_node_value(&root_node).to_string()
}

