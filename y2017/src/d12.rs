use std::collections::{HashMap, HashSet, VecDeque};

use crate::inputs::read_content;

struct Graph {
    edges: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new(input: &str) -> Graph {
        let mut edges = HashMap::new();

        for line in input.lines() {
            let (node, nbrs) = line.split_once("<->").unwrap();
            let node: usize = node.trim().parse().unwrap();

            let node_edges = edges.entry(node).or_insert(HashSet::new());
            let mut nbr_nodes = Vec::new();

            for nbr in nbrs.split(",") {
                let nbr = nbr.trim().parse::<usize>().unwrap();
                if nbr == node {
                    continue;
                }

                node_edges.insert(nbr);
                nbr_nodes.push(nbr);
            }

            for nbr in nbr_nodes {
                edges.entry(nbr)
                    .and_modify(|ne| { ne.insert(node); })
                    .or_insert(HashSet::from([node]));
            }
        }

        Graph { edges }
    }

    fn find_all_connected_nodes(&self, start: usize) -> HashSet<usize> {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([start]);

        while let Some(curr) = queue.pop_front() {
            if seen.contains(&curr) {
                continue;
            }

            seen.insert(curr);
            if let Some(nbrs) = self.edges.get(&curr) {
                for nbr in nbrs {
                    if !seen.contains(nbr) {
                        queue.push_back(*nbr);
                    }
                }
            }
        }

        seen
    }

    fn count_num_connected_neighbours(&self, start: usize) -> usize {
        let connected = self.find_all_connected_nodes(start);
        connected.len()
    }

    fn count_connected_groups(&self) -> usize {
        let mut nodes = self.edges.keys().map(|k| *k).collect::<HashSet<usize>>();

        let mut count = 0;

        while nodes.len() > 0 {
            count += 1;
            let start = *nodes.iter().next().unwrap();

            let mut seen = HashSet::new();
            let mut queue = VecDeque::from([start]);

            while let Some(curr) = queue.pop_front() {
                if seen.contains(&curr) {
                    continue;
                }

                seen.insert(curr);
                nodes.remove(&curr);
                if let Some(nbrs) = self.edges.get(&curr) {
                    for nbr in nbrs {
                        if !seen.contains(nbr) {
                            queue.push_back(*nbr);
                        }
                    }
                }
            }
        }

        count
    }
}

pub fn solve_a() {
    let input = read_content(12);
    let graph = Graph::new(&input);
    let ans = graph.count_num_connected_neighbours(0);

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let input = read_content(12);
    let graph = Graph::new(&input);
    let ans = graph.count_connected_groups();

    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use crate::d12::Graph;

    #[test]
    fn test_count_num_connected_neighbours() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        let graph = Graph::new(input);
        assert_eq!(graph.count_num_connected_neighbours(0), 6);
    }

    #[test]
    fn test_count_connected_groups() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        let graph = Graph::new(input);
        assert_eq!(graph.count_connected_groups(), 2);
    }
}