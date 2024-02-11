use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use regex::Regex;

use crate::inputs::read_content;

pub fn solve_a() {
    let top_node = form_graph(&read_content(7));

    let ans = top_node.borrow().name.clone();
    println!("Solution A: {}", ans);
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: usize,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &str, weight: usize) -> Node {
        Node { name: name.to_string(), weight, parent: None, children: Vec::new() }
    }

    fn new_rc(name: &str, weight: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(name, weight)))
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }

    fn find_imbalanced_node(&self) -> (Option<usize>, usize) {
        let mut total_weight = self.weight;
        let mut child_weights = HashMap::new();
        for c in self.children.iter() {
            let (imb, child_weight) = c.borrow().find_imbalanced_node();

            if imb.is_some() {
                return (imb, 0);
            }

            child_weights.entry(child_weight)
                .and_modify(|v: &mut Vec<Rc<RefCell<Node>>>| {
                    v.push(c.clone());
                })
                .or_insert(vec![c.clone()]);

            total_weight += child_weight;
        }

        if child_weights.len() > 1 {
            let (&problem_weight, problem_children) = child_weights.iter()
                .find(|(_, cv)| {
                    cv.len() == 1
                }).unwrap();

            // problem child weight
            let pc_weight = problem_children.first().unwrap().borrow().weight;

            let (&ideal_weight, _) = child_weights.iter()
                .find(|(_, cv)| cv.len() > 1)
                .unwrap();

            let w = if ideal_weight < problem_weight {
                pc_weight - (problem_weight - ideal_weight)
            } else {
                pc_weight + (ideal_weight - problem_weight)
            };

            (Some(w), 0)
        } else {
            (None, total_weight)
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        other.name == self.name
    }
}

impl Eq for Node {}

fn form_graph(input: &str) -> Rc<RefCell<Node>> {
    let node_re = Regex::new(r"(\w+) \((\d+)\)").unwrap();

    let mut nodes_map = HashMap::new();
    let mut node_child_map = HashMap::new();


    for (node, children) in input.lines()
        .map(|line| {
            let (node_line, children_line) = if line.contains(" -> ") {
                line.split_once(" -> ").unwrap()
            } else {
                (line, "")
            };

            let captures = node_re.captures(node_line.trim()).unwrap();

            let name = captures.get(1).unwrap().as_str();
            let weight = captures.get(2).unwrap().as_str().parse::<usize>().expect(format!("Could not parse: '{}'", line).as_str());

            let children = if children_line.is_empty() {
                Vec::new()
            } else {
                children_line.split(", ").map(|x| x.to_string()).collect()
            };

            (Node::new_rc(name, weight), children)
        }) {
        let key = node.borrow().name.clone();

        nodes_map.insert(key.clone(), node);
        node_child_map.insert(key, children);
    }

    for (key, node) in &nodes_map {
        let children = node_child_map.get(key).unwrap();

        let mut m_node = node.borrow_mut();
        for child_key in children {
            let child_node = nodes_map.get(child_key).unwrap();

            m_node.add_child(child_node.clone());
            let mut m_c = child_node.borrow_mut();
            m_c.parent = Some(node.clone());
        }
    }

    for node in nodes_map.values() {
        if node.borrow().parent.is_none() {
            return node.clone();
        }
    }

    panic!("All nodes have parents!")
}

pub fn solve_b() {
    let root = form_graph(&read_content(7));

    let (ans, _) = root.borrow().find_imbalanced_node();

    assert!(ans.is_some(), "Could not find solution");
    println!("Solution B: {}", ans.unwrap());
}