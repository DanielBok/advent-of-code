use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet, VecDeque};

type Graph = HashMap<char, HashMap<char, usize>>;

const PUZZLE_INPUT: &str = "#################################################################################
#...#.......#...U...#.....#...........#.#.....#.......#...#...#.......#.........#
#.###.#.#####.###.#.#.#####.###.#####.#.#.###.#.#.###H#.#.#.###.#.#####.#######.#
#p..#.#...#...#.#.#...#......c#.#...#...#...#.#.#.#.....#.#.#...#...#...#...#...#
#.#.#N###.#.###.#.#####.#######.#.#.###.###.#.#.#.#######.#I#.#####.#.###.#.#.#.#
#.#.#...#.......#.#.....#.....#...#.#...#...#.#.#.#.....#.#.#.#...#...#...#.#.#.#
###.###.#########.#.#####.###.#####.#.###.###.###.#.###.#.#.#.#.#.#######.#.#.###
#...#...#.#.T.#...#...#...#.#.......#...#...#.....#...#.#...#...#.#..q..#.#.#...#
#.###.###.#.#.#.###.###.###.###.#######.#.#.#########.#.#########.#.###.#.#.###.#
#.#...#...#.#...#...#...#...#...#...#...#.#.........#.#.........#...#.#...#...#.#
#.#.#####.#.#####.###.###.###.###.#.###.#.#########.#.#######.#######.#######.#.#
#...#.....#.....#.#.#...#..b..#...#e..#.#.#...#...#...#.....#.......#.....#...#.#
#.###.###.#####.#.#.###.#######.#####.#.#.#.###.#.#######.#.#######.#.###.#B###.#
#.#...#.#.#...#.#.....#.............#.#.#.#.....#.........#.W.....#.#.#.#.....#.#
#.###.#.#.#.#.#.#####.###############.#.#.#########.#.#######.#####.#.#.#######.#
#.....#...#.#.#...#...#.....#.........#.#.....#...#.#...#.....#...#.#.#.....#...#
#######.###.#K###.#.###.#####.#########.#####.#.#.#####.#######.#.#.#.#.#.#.#.#.#
#.......#...#...#...#.......#...#...#.#.#.#...#.#.......#....d#g#.#...#.#.#.#.#.#
#.#######.#####.#########.#.###.#A#.#.#.#.#.###.#######.#.###.#.#.###.###.###.#.#
#.#...#...#...#.#.......#.#.#...#.#.#...#...#...#.......#.#...#.#...#.#...#...#.#
#.#.#.###.#.###.###.###.###.#.#.#.#.#####.###.#.#########.#.###.###.#.#.#.#.###.#
#...#.....#.....#...#.#.#...#.#.#.#.#...#.#...#.....#...#.#...#.#...#...#.#...#j#
###########.#####.###.#.#.#.#.#.#.#.#.#.#.#######.#.#.#.#.#####.#V###########.###
#.....#.F...#..m..#...#...#.#.#.#.#...#.#.......#.#.#.#.#.#..r#.#.#.........#...#
###Y#.#.#####.###.#.#####.###.###.#####.#######.#.#.#.#.#.#.#.#.#.###.#####.###.#
#...#.#...#...#v#.#.....#.#...#...#.L...#...#...#.#...#.#...#...#...#.#...#.#...#
#.#.#####.#.###.#.#####.###.###.###.###.#.###.#########.###########.#.#.###.#.#.#
#.#.#.....#.#.....#...#...#.......#.#.#.#.....#.......#.........#...#.#...#.#.#.#
#.#.#.#####X#.#####.#.###.#.#######.#.#.#.#####.#####.#.#########.###.###.#.#.#.#
#.#.#.......#.......#.#...#...#.....#...#.#.......#...#.....#.....#...Z.#...#.#.#
#.###################.#.#####.#.#####.###.#######.#.###.###.#.#####.###.#.###.#.#
#.#..............y..#.#.....#.#.#...#...#.........#.#...#...#.#...#.#.#.#.....#.#
#.#D###.###########.#.#####.###.#.#####.###########.#####.#.#.###.#.#.#.###.#####
#.#.#...#...........#.#.....#...#.....#.#..s....#...#...#.#.#...#...#.#...#.#...#
#.#.#.#######.#######.#.#####.###.###.#.#.###.###.###.#.#.#.###.#.###.###.###.#.#
#...#.#.....#.#.......#.#.....#.#.#.#.#.#...#.....#...#...#.#...#.......#k..#.#.#
#.###.#.###.#.#.#######.#.#.###.#.#.#.#.###.#######.#####.###.#############.#.#.#
#...#.R.#...#.#...#...#.#.#...#.#...#...#...#.......#.#...#...#.....#.....#...#.#
###.#####.#######.###.#.#####.#.###.#####.#######.###.#.###.###.###.#.###.#####.#
#.......#.....G.......#.......#...................#.......#.....#..x..#......f..#
#######################################.@.#######################################
#...#o..........#...#.................#.........#.....#...................#.....#
#.###.#.#######.#.#.#.###############.#.#.#.#####.#.#.#.#############.###.#.###.#
#.....#.#.....#.#.#...#.....#.......#.#.#.#.......#.#.#.....#.......#...#...#...#
#.#####.###.###.#.#####.#####.#####.#.#.#.#########.#######.#.#####.#########.#.#
#.#...#...#...#.#...#...#.....#.#...#...#.#.......#.........#.#.#...#.......#.#.#
#.###.###.###.#.###.#.#.#.#####.#.#####.#.#######.###########.#.#.###.#####.#.#.#
#...#...#...#.#.#...#.#.#...#...#.......#...#.....#.....#.....#.......#...#.#.#.#
###.#.#.###.#.#.#.###.#####.#.#.###########.#.#####.###.#.#.###########.#.#.#.###
#.....#.#...#.#...#.#.......#.#.........#...#.......#...#.#.#...#.......#.#.#...#
#########.###.#####.#.#########.#######.#.###.#######.###.###.#.###.#.#####.#.#.#
#.......#.#.....#.....#.......#.#.....#.#.#.#...#...#...#.....#...#.#.#.....#.#.#
#.#####.#.#.###.#.#########.#.#.#.###.#.#.#.###.###.###.#########.#.#.#.#######.#
#...#.....#.#.#...#.......#.#...#.#.#...#.#...#.......#...#.....#.#.#.#.#.......#
#.#.#######.#.#####.#####.#.#####.#.#####.#.#########.###.#.###.#.###.#.#.###.#O#
#.#.#...#.....#.....#...#...#..i#.#...#.#.#.........#.#...#.#...#.#...#.#.#...#.#
###.#.#.#####.###.#####.#####.###.#.#.#.#.###.#.#####.#.###.#.###.#.###.#.#.###.#
#...#.#...#.......#.........#.....#.#.#.#...#.#.......#.#...#...#.#.....#.#...#.#
#.###.###.###########.#######.#####.#.#.###.#.#######.#.#.#####.#.#####.###.#.#.#
#.....#.#...........#...#...#.....#.#.#.#.S.#...#...#.#.#.....#.#.#...#...#.#.#.#
#######.###########.###.#.#.#####.#.#.#.#.###.###.#.###.###.###M#.#.#.###.###.#.#
#.......#.....#...#...#.#.#.....#...#...#...#.#...#.......#.#.#.#...#.#.#.....#.#
#.#.#####.#.#.###.#####.#.#####.#######.###.#.#.#####.#####.#.#.#####.#.###.#####
#.#.....#.#.#.......#.....#...#.......#.#..l#.#.....#.#.....#.#.#...C.#.....#...#
#.#####.#.#.#######.#.#####.#.#####.###.#.#####.###.#.#.#####.#.#.#########.#.#.#
#.#...#z#.#.#.....#.#...#...#.....#.....#.....#.#..w#.#.#...#...#.#.......#.#.#.#
#.#.#J#.#.#.#.#####.###.#.#######.###########.#.#.#####.#.#.#####.#.#####.###.#.#
#.#.#.#...#.#.#.....#...#...#...#.....#.#.....#.#.....#...#...#...#.....#...#.#.#
###.#.#####.#.#.#####.#####.#.###.###.#.#.#####.#####.#######.#.#######.###.#.#.#
#...#.....#.#...#.#...#.#...#...#...#.#.#a#.....#.........#.#.#.......#.#.#...#n#
#.#######.#.#.###.#Q###.#.#####P###.#.#.#.#######.#######.#.#.#.###.###.#.#####.#
#.#.........#.....#.#...#...#...#.#.#...#.#...#...#.....#.#.#.#...#.#.....#...#.#
#.#################.#.#.###.#.#.#.#.#####.#.#.#.###.###.#.#.#.#####.#.#####.#.#.#
#.#.........#.....#.#.#.#.....#...#.....#...#.#.#.#...#.....#.....#.....#...#...#
#.#.#######.#.###.#.###.#.#######.#####.#####.#.#.###.###########.#######.#######
#...#.....#.#.#.#.#...#...#.....#...#.#.#...#...#...#...#.#.....#.#.....#...#...#
#.#####.###.#.#.#.###.#####.###.###.#.#.#.#######.#####.#.#.###.#.#.###.#.#.#.#.#
#...#...#...#.#.#...#.#.....#.#.#.....#.#.......#.....#...#.#.#...#...#.#.#...#.#
###.#.#.#.###.#.#.###.#.#####.#.#######.#.#####.#.###.###.#.#.#######.#.#######.#
#.....#.#.......#....t..#......u........#.....#..h..#...E.#...........#.........#
#################################################################################";

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        let x = self.0;
        let y = self.1;

        let mut nbr = vec![
            Point(x, y + 1),
            Point(x + 1, y),
        ];

        if x > 0 {
            nbr.push(Point(x - 1, y));
        }
        if y > 0 {
            nbr.push(Point(x, y - 1));
        }

        nbr
    }
}

#[derive(PartialEq)]
enum Tile {
    Floor,
    Wall,
    Node(char),
}

pub fn solve_a() {
    let graph = parse_map_to_graph(PUZZLE_INPUT);

    if let Some(ans) = search(graph, '@') {
        println!("Solution A: {}", ans);
    } else {
        panic!("A: Could not find solution");
    }
}


fn parse_map_to_graph(input: &str) -> Graph {
    let grid = parse_map_to_grid(input);
    parse_grid_to_graph(grid)
}


fn parse_map_to_grid(input: &str) -> HashMap<Point, Tile> {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Floor,
                _ => Tile::Node(c)
            };
            grid.insert(Point(x, y), tile);
        }
    }

    grid
}

fn parse_grid_to_graph(grid: HashMap<Point, Tile>) -> Graph {
    let mut graph = HashMap::new();

    for (pt, tile) in &grid {
        if let Tile::Node(c) = tile {
            graph.insert(*c, get_neighbour_nodes(&grid, *pt));
        }
    }

    graph
}

fn get_neighbour_nodes(grid: &HashMap<Point, Tile>, initial: Point) -> HashMap<char, usize> {
    let mut nbr = HashMap::new();
    let mut visited = HashSet::from([initial]);
    let mut queue = VecDeque::from([(initial, 0)]);

    while let Some((pt, step)) = queue.pop_front() {
        for next in pt.neighbours() {
            match grid.get(&next) {
                Some(tile) => {
                    match tile {
                        Tile::Floor => {
                            if !visited.contains(&next) {
                                queue.push_back((next, step + 1))
                            }
                        }
                        Tile::Node(c) => {
                            nbr.insert(*c, step + 1);
                        }
                        _ => {}
                    }
                }
                None => {}
            }
            visited.insert(next);
        }
    }

    nbr
}

#[derive(PartialEq, Eq)]
struct SearchState {
    steps: usize,
    node: char,
    keys: BTreeSet<char>,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SearchState {
    fn cache_key(&self) -> (char, BTreeSet<char>) {
        (self.node, self.keys.clone())
    }
}


fn search(graph: Graph, node: char) -> Option<usize> {
    let num_keys = graph.iter().filter(|(c, _)| c.is_ascii_lowercase()).count();

    // best distance given position (node) and types of keys collected (set of keys)
    let mut distances: HashMap<(char, BTreeSet<char>), usize> = HashMap::new();

    let mut queue = BinaryHeap::from([SearchState {
        steps: 0,
        node,
        keys: BTreeSet::new(),
    }]);

    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(current) = queue.pop() {
        if current.keys.len() == num_keys {
            return Some(current.steps);
        }

        let cache_key = current.cache_key();
        if let Some(&best_num_steps) = distances.get(&cache_key) {
            if current.steps > best_num_steps {
                continue;
            }
        }

        // gets a vector of keys and the respective distance from the current node
        let entry = cache.entry(cache_key)
            .or_insert_with(|| get_reachable_keys(&graph, &current.keys, current.node));

        for &mut (next_node, cost) in entry {
            let mut next_keys = current.keys.clone();
            next_keys.insert(next_node);
            let next_step = current.steps + cost;

            // recorded node distance, if does not exist, use MAX
            let node_distance = distances.entry((next_node, next_keys.clone())).or_insert(usize::MAX);

            if next_step < *node_distance {
                *node_distance = next_step; // update distance

                queue.push(SearchState {
                    steps: next_step,
                    node: next_node,
                    keys: next_keys,
                });
            }
        }
    }

    None
}

#[derive(PartialEq, Eq)]
struct DjikstraState {
    cost: usize,
    node: char,
}

// make it comparable for the max-heap implementation
// since we want the items that have travelled the least to be on the top of the heap
// we need to "invert" the cost, so if other > self, self will be "Greater" and go on top
impl Ord for DjikstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then(self.node.cmp(&other.node))
    }
}

impl PartialOrd for DjikstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Searches for all keys that can be reached from current node. Uses the Djikstra algorithm
fn get_reachable_keys(graph: &Graph, keys: &BTreeSet<char>, start: char) -> Vec<(char, usize)> {
    let mut dist = graph.keys().map(|c| (*c, usize::MAX)).collect::<HashMap<char, usize>>();

    *dist.get_mut(&start).unwrap() = 0;
    let mut heap = BinaryHeap::from([
        DjikstraState { cost: 0, node: start }
    ]);

    // keys that can be reached
    let mut reach = HashSet::new();

    while let Some(DjikstraState { cost, node }) = heap.pop() {
        if node.is_ascii_lowercase() && !keys.contains(&node) {
            // can reach this key
            reach.insert(node);
            continue;
        }

        if cost > dist[&node] {
            // already found a better solution, skip this search route
            continue;
        }

        for (&next_node, &next_cost) in graph.get(&node).unwrap() {
            if next_node.is_ascii_uppercase() && !keys.contains(&next_node.to_ascii_lowercase()) {
                // don't have key to door, can't pass
                continue;
            }

            let next_cost = cost + next_cost;
            if next_cost < dist[&next_node] {
                // update the distance map, set the next node cost to this lower cost
                dist.insert(next_node, next_cost);

                // add the state back into the heap to search again
                heap.push(DjikstraState {
                    cost: next_cost,
                    node: next_node,
                });
            }
        }
    }

    reach.iter().map(|node| (*node, dist[&node])).collect()
}


pub fn solve_b() {
    let graph = parse_map_to_graph(&modify_input(PUZZLE_INPUT));

    if let Some(ans) = quadrant_search(&graph) {
        println!("Solution B: {}", ans);
    } else {
        panic!("B: Could not find solution");
    }
}


fn modify_input(input: &str) -> String {
    let mut lines = input.lines().map(|e| e.to_string()).collect::<Vec<_>>();

    let (row, col) = 'inp: {
        for (i, line) in lines.iter().enumerate() {
            if line.contains("@") {
                let row = i;
                let col = line.char_indices().find_map(|(idx, c)| { if c == '@' { Some(idx) } else { None } }).unwrap();
                break 'inp (row, col);
            }
        }
        panic!("Could not find row and col index to change")
    };

    lines[row].replace_range(col - 1..=col + 1, "###");
    lines[row - 1].replace_range(col - 1..=col + 1, "@#$");
    lines[row + 1].replace_range(col - 1..=col + 1, "%#*");

    lines.join("\n")
}


#[derive(PartialEq, Eq)]
struct QuadrantSearchState {
    steps: usize,
    robots: [char; 4],
    keys: BTreeSet<char>,
}

impl Ord for QuadrantSearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for QuadrantSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl QuadrantSearchState {
    fn cache_key(&self) -> ([char; 4], BTreeSet<char>) {
        (self.robots, self.keys.clone())
    }
}


fn quadrant_search(graph: &Graph) -> Option<usize> {
    let num_keys = graph.iter().filter(|(c, _)| c.is_ascii_lowercase()).count();

    let robots = ['@', '$', '%', '*'];

    // best distance given position (node) and types of keys collected (set of keys)
    let mut distances: HashMap<([char; 4], BTreeSet<char>), usize> = HashMap::from([
        ((robots.clone(), BTreeSet::new()), 0)
    ]);

    let mut queue = BinaryHeap::from([
        QuadrantSearchState {
            steps: 0,
            robots,
            keys: BTreeSet::new(),
        }
    ]);


    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(current) = queue.pop() {
        if current.keys.len() == num_keys {
            return Some(current.steps);
        }

        if let Some(&best_num_steps) = distances.get(&current.cache_key()) {
            if current.steps > best_num_steps {
                continue;
            }
        }

        for (robot_num, &robot_loc) in current.robots.iter().enumerate() {
            // get every reachable nodes for current robot
            let entry = cache.entry((robot_loc, current.keys.clone()))
                .or_insert_with(|| get_reachable_keys(&graph, &current.keys, robot_loc));

            for &mut (next_node, cost) in entry {
                // for each reachable node for the robot, set the robot's next location to this node
                // and update its keys
                let mut next_keys = current.keys.clone();
                next_keys.insert(next_node);

                // this lets the robot update its own position while keeping the other robot's position fixed
                let mut next_robots = current.robots.clone();
                next_robots[robot_num] = next_node;

                // next distance cost
                let next_steps = current.steps + cost;

                // check if we have seen the next state
                let distance = distances
                    .entry((next_robots.clone(), next_keys.clone()))
                    .or_insert(usize::MAX);

                // if we haven't seen the next state or the next state we saved previously is not as good
                // as the current one we found, we search out the next state
                if next_steps < *distance {
                    *distance = next_steps;
                    queue.push(QuadrantSearchState {
                        steps: next_steps,
                        robots: next_robots,
                        keys: next_keys,
                    });
                }
            }
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use crate::d18::{parse_map_to_graph, search};

    #[test]
    fn test_part_one() {
        for (inp, exp) in [
            ("#########
#b.A.@.a#
#########", 8),
            ("########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################", 86),
            ("########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################", 132),
            ("#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################", 136),
            ("########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################", 81)
        ] {
            let graph = parse_map_to_graph(inp);

            let ans = search(graph, '@');
            assert_eq!(ans, Some(exp));
        }
    }

    #[test]
    fn test_part_two() {
        for (inp, _exp) in [
            ("#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######", 8)
        ] {
            let input = modify_input(inp);
            dbg!(input);
        }
    }

    fn modify_input(input: &str) -> String {
        let mut input = input.to_string();
        let mut changes = vec!["$", "%", "*"];

        while let Some(token) = changes.pop() {
            if input.contains("@") {
                input = input.replacen("@", token, 1);
            }
        }

        input
    }
}