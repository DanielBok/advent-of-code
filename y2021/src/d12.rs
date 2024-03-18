use std::collections::{HashMap, HashSet, VecDeque};

use crate::inputs::read_contents;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    value: String,
    is_multi: bool,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let is_multi = value.chars().all(|c| c.is_uppercase());

        Self { value: value.to_string(), is_multi }
    }
}


fn form_graph(input: &str) -> HashMap<Node, Vec<Node>> {
    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();

    input.lines()
         .for_each(|line| {
             let (from_, to_) = line.split_once('-')
                                    .map_or(None, |(a, b)| {
                                        Some((Node::from(a), Node::from(b)))
                                    })
                                    .unwrap();

             graph.entry(from_.clone())
                  .and_modify(|v| v.push(to_.clone()))
                  .or_insert_with(|| vec![to_.clone()]);

             graph.entry(to_)
                  .and_modify(|v| v.push(from_.clone()))
                  .or_insert_with(|| vec![from_]);
         });

    graph
}


pub fn solve_a() {
    let graph = form_graph(&read_contents(12));
    let ans = count_number_of_pathways(&graph);

    println!("Solution A: {}", ans);
}

fn count_number_of_pathways(graph: &HashMap<Node, Vec<Node>>) -> usize {
    let end = Node::from("end");
    let start = Node::from("start");
    let mut queue = VecDeque::from([(start.clone(), HashSet::from([start]))]);

    let mut total = 0;
    while let Some((current, mut visited)) = queue.pop_front() {
        if current == end {
            total += 1;
            continue;
        }

        if !current.is_multi {
            visited.insert(current.clone());
        }

        if let Some(nodes) = graph.get(&current) {
            for node in nodes {
                if visited.contains(node) { continue; }
                queue.push_back((node.clone(), visited.clone()));
            }
        }
    }

    total
}

pub fn solve_b() {
    let graph = form_graph(&read_contents(12));
    let ans = count_number_of_pathways2(&graph);

    println!("Solution B: {}", ans);
}

fn count_number_of_pathways2(graph: &HashMap<Node, Vec<Node>>) -> usize {
    let end = Node::from("end");
    let start = Node::from("start");

    let mut queue = VecDeque::from([(start.clone(), HashSet::new(), false)]);

    let mut total = 0;
    while let Some((current, mut visited, has_twice)) = queue.pop_front() {
        let next_has_twice = if visited.contains(&current) {
            if has_twice { continue; } else { true }
        } else {
            has_twice
        };

        if !current.is_multi {
            visited.insert(current.clone());
        }

        if let Some(nodes) = graph.get(&current) {
            for node in nodes {
                if *node == end {
                    total += 1;
                    continue;
                }
                if (*node == start) || (has_twice && visited.contains(node)) { continue; }

                queue.push_back((node.clone(), visited.clone(), next_has_twice));
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::{count_number_of_pathways, count_number_of_pathways2, form_graph};

    #[test]
    fn test_count_number_of_pathways() {
        for (inp, exp) in [
            ("start-A
start-b
A-c
A-b
b-d
A-end
b-end", 10),
            ("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc", 19),
            ("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW", 226)
        ] {
            let graph = form_graph(inp);
            let ans = count_number_of_pathways(&graph);
            assert_eq!(ans, exp);
        }
    }

    #[test]
    fn test_count_number_of_pathways2() {
        for (inp, exp) in [
            ("start-A
start-b
A-c
A-b
b-d
A-end
b-end", 36),
            ("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc", 103),
            ("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW", 3509)
        ] {
            let graph = form_graph(inp);
            let ans = count_number_of_pathways2(&graph);
            assert_eq!(ans, exp);
        }
    }
}
