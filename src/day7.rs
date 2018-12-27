use std::fmt::{Display, Debug};
use regex::Regex;
use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::cmp;
use itertools::Itertools;

// #![derive(PartialEq, Eq)]
struct Node<'a> {
    id: String,
    dependent_nodes: RefCell<Vec<&'a Node<'a>>>,
    required_nodes_count: Cell<i32>
}

impl<'a> Node<'a> {
    pub fn new<'b>(id: String) -> Node<'b> {
        Node {id, dependent_nodes: RefCell::from(Vec::new()), required_nodes_count: Cell::new(0)}
    }

    pub fn add(&self, other: &'a Node<'a>) {
        let mut deps = self.dependent_nodes.borrow_mut();
        deps.push(other);
        other.required_nodes_count.set(other.required_nodes_count.get() + 1);
    }

    pub fn release_one_required_node(&self) -> bool {
        let new_other_required_count = self.required_nodes_count.get() - 1;
        self.required_nodes_count.set(new_other_required_count);
        new_other_required_count == 0
    }

    pub fn release_dependencies(&self) -> Vec<&'a Node> {
        let mut deps = self.dependent_nodes.borrow_mut();
        deps.drain(..)
            .filter(|other| other.release_one_required_node())
            .collect()
    }

    pub fn has_dependencies(&self) -> bool {
        self.required_nodes_count.get() != 0
    }
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(&self.id)
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(&format!("{}: {} incoming, outgoing to {:?}", &self.id, &self.required_nodes_count.get(), &self.dependent_nodes.borrow()))
    }
}

// struct ToposortIterator<'a> {
//     nodes: HashMap<String, Node<'a>>,
//     next_nodes: Vec<&'a Node<'a>>
// }

// impl<'a> ToposortIterator<'a> {
//     fn new() -> ToposortIterator<'a> {
//         ToposortIterator { nodes: HashMap::new(), next_nodes: Vec::new()}
//     }

//     fn initialize(&'a mut self, edges: &Vec<(String, String)>) {
//         for (source, target) in edges {
//             self.nodes.entry(source.clone())
//                 .or_insert(Node::new(source.clone()));
//             self.nodes.entry(target.clone())
//                 .or_insert(Node::new(target.clone()));
//         }

//         for (source, target) in edges {
//             self.nodes.get(source).unwrap().add(self.nodes.get(target).unwrap());
//         }

//         self.next_nodes = self.nodes.values().filter(|v| !v.has_dependencies()).collect::<Vec<&Node>>();
//     }

//     fn next_ref(&'a mut self) -> Option<&'a Node<'a>> {
//         if self.next_nodes.len() == 0 {
//             None
//         } else {
//             self.next_nodes.sort_by_key(|node| cmp::Reverse(&node.id));
//             let node = self.next_nodes.pop().unwrap();
//             let mut free_nodes = node.release_dependencies();
//             self.next_nodes.append(&mut free_nodes);
//             Some(node)
//         }
//     }
// }

// impl<'a> Iterator for ToposortIterator<'a> {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next().map(|n| n.id)
//     }
// }

// impl<'a> Iterator for ToposortIterator<'a> {
//     type Item=&'a Node<'a>;

//     fn next(&'a mut self) -> Option<&'a Node<'a>> {
//         None
//     }
// }

pub fn get_edges(lines: &Vec<String>) -> Vec<(String, String)> {
    let parser_regex = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    lines.iter()
        .map(|line| {
            let captures = parser_regex.captures(line).unwrap();
            (captures[1].to_string(), captures[2].to_string())
        })
        .collect()
}

fn toposort(edges: &Vec<(String, String)>) -> Vec<String> {
    let mut nodes = HashMap::new();
    for (source, target) in edges {
        nodes.entry(source.clone())
            .or_insert(Node::new(source.clone()));
        nodes.entry(target.clone())
            .or_insert(Node::new(target.clone()));
    }

    for (source, target) in edges {
        nodes.get(source).unwrap().add(nodes.get(target).unwrap());
    }

    let mut toposorted_nodes = Vec::new();
    let mut next_nodes = nodes.values().filter(|v| !v.has_dependencies()).collect::<Vec<&Node>>();
    while next_nodes.len() > 0 {
        next_nodes.sort_by_key(|node| cmp::Reverse(&node.id));
        let node = next_nodes.pop().unwrap();
        toposorted_nodes.push(node.id.clone());
        let mut free_nodes = node.release_dependencies();
        next_nodes.append(&mut free_nodes);            
    }

    toposorted_nodes
}

pub fn star1(lines: &Vec<String>) -> String {
    let edges = get_edges(lines);
    let toposorted_ids = toposort(&edges);

    toposorted_ids.iter().join("") 
}

pub fn get_time_needed(s: &String) -> i32 {
    (s.as_bytes()[0] - "A".as_bytes()[0] + 60 + 1) as i32
}

pub fn star2(lines: &Vec<String>) -> String {
    const WORKER_CAPACITY: usize = 5;
    let mut workers: Vec<(i32, &Node)> = Vec::new();

    let edges = &get_edges(lines);
    let mut nodes = HashMap::new();
    for (source, target) in edges {
        nodes.entry(source.clone())
            .or_insert(Node::new(source.clone()));
        nodes.entry(target.clone())
            .or_insert(Node::new(target.clone()));
    }

    for (source, target) in edges {
        nodes.get(source).unwrap().add(nodes.get(target).unwrap());
    }

    let mut next_nodes = nodes.values().filter(|v| !v.has_dependencies()).collect::<Vec<&Node>>();
    let mut time_needed = 0;
    while next_nodes.len() > 0 || workers.len() > 0 {
        next_nodes.sort_by_key(|node| cmp::Reverse(&node.id));
        // Fill the free worker slots with nodes
        for _i in 0..cmp::min(WORKER_CAPACITY - workers.len() + 1, next_nodes.len()) {
            // Using indexing approach instead of iterators (as drain_filter is still unstable)
            let node = next_nodes.remove(next_nodes.len() - 1);
            workers.push((get_time_needed(&node.id), &node));
        }

        let min_time_needed = workers.iter().min_by_key(|&(time_needed, _)| time_needed).unwrap().0;
        for mut worker_tuple in workers.iter_mut() {
            worker_tuple.0 -= min_time_needed;
        }
        time_needed += min_time_needed;

        for i in (0..workers.len()).rev() {
            if let (0, finished_node) = workers[i] {                
                let mut free_nodes = finished_node.release_dependencies();
                next_nodes.append(&mut free_nodes);
                workers.remove(i);
            }
        }
    }

    time_needed.to_string() 
}