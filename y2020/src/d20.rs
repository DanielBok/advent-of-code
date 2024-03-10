use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};

use itertools::{Itertools, MinMaxResult};

use crate::inputs::read_contents;

type Image = Vec<Vec<char>>;


#[derive(Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Side::Top => "Top",
            Side::Bottom => "Bottom",
            Side::Left => "Left",
            Side::Right => "Right",
        })
    }
}

#[derive(Clone)]
struct Tile {
    id: usize,
    image: Image,
    sides: HashMap<String, String>,
}


impl From<&str> for Tile {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let id = lines.next().unwrap()
                      .trim_end_matches(':')
                      .split_ascii_whitespace()
                      .skip(1).next().unwrap()
                      .parse().unwrap();

        let image: Image = lines.map(|line| line.chars().collect()).collect();
        let mut sides = HashMap::new();

        for (_, edge) in get_image_sides(&image) {
            let reverse = edge.chars().rev().collect::<String>();
            sides.insert(edge.clone(), reverse.clone());
            sides.insert(reverse, edge);
        }

        Tile { id, image, sides }
    }
}


fn get_image_sides(image: &Image) -> [(Side, String); 4] {
    [
        (Side::Top, get_image_side(image, Side::Top)),
        (Side::Bottom, get_image_side(image, Side::Bottom)),
        (Side::Left, get_image_side(image, Side::Left)),
        (Side::Right, get_image_side(image, Side::Right)),
    ]
}

fn get_image_side(image: &Image, side: Side) -> String {
    match side {
        Side::Top => image.iter().next().unwrap().iter().collect::<String>(),
        Side::Bottom => image.iter().last().unwrap().iter().collect::<String>(),
        Side::Left => image.iter().map(|v| v.iter().next().unwrap()).collect::<String>(),
        Side::Right => image.iter().map(|v| v.iter().last().unwrap()).collect::<String>(),
    }
}


fn parse_input(input: &str) -> Vec<Tile> {
    let input = input.replace("\r", "");

    input.split("\n\n")
         .map(Tile::from)
         .collect()
}

pub fn solve_a() {
    let tiles = parse_input(&read_contents(20));

    let ans = find_corner_multiple(tiles);
    println!("Solution A: {}", ans);
}

fn find_corner_multiple(tiles: Vec<Tile>) -> usize {
    let mut tile_connections: HashMap<usize, usize> = tiles.iter().map(|t| (t.id, 0)).collect();

    let edge_map = tiles.iter().fold(HashMap::<String, Vec<usize>>::new(), |mut acc, t| {
        t.sides.iter().for_each(|(s, _)| {
            acc.entry(s.clone())
               .and_modify(|v| v.push(t.id))
               .or_insert(vec![t.id]);
        });
        acc
    });

    for ids in edge_map.values() {
        if ids.len() == 2 {
            for id in ids {
                *tile_connections.get_mut(id).unwrap() += 1;
            }
        } else if ids.len() > 2 {
            panic!("There shouldn't be 3 tiles that match on the same edge");
        }
    }

    // we double count edges due to flips
    tile_connections.into_iter()
                    .filter(|(_, v)| *v == 4)
                    .fold(1, |acc, (k, _)| acc * k)
}

pub fn solve_b() {
    let tiles = parse_input(&read_contents(20));
    let image = piece_puzzle(tiles);

    let ans = count_rough_waters(&image);
    println!("Solution B: {}", ans);
}


fn piece_puzzle(tiles: Vec<Tile>) -> Image {
    let mut edge_map = tiles.iter().fold(HashMap::<String, Vec<usize>>::new(), |mut acc, t| {
        t.sides.iter().for_each(|(s, _)| {
            acc.entry(s.clone())
               .and_modify(|v| v.push(t.id))
               .or_insert(vec![t.id]);
        });
        acc
    });

    let tiles_id_map: HashMap<usize, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();
    let mut positions: HashMap<(i32, i32), Image> = HashMap::from([((0, 0), tiles[0].image.clone())]);
    let mut tidmap = HashMap::new();

    let mut queue = VecDeque::from([(0, 0, tiles[0].id)]);

    while let Some((x, y, tile_id)) = queue.pop_front() {
        let image = positions.get(&(x, y)).unwrap();
        tidmap.insert((x, y), tile_id);

        for (side, edge) in get_image_sides(image) {
            let (nx, ny) = match side {
                Side::Top => (x, y + 1),
                Side::Bottom => (x, y - 1),
                Side::Left => (x - 1, y),
                Side::Right => (x + 1, y),
            };

            if positions.contains_key(&(nx, ny)) {
                // position already taken, don't need to check anymore
                continue;
            }

            // get the conjoining vector. If this edge is paired, then the vector should have only
            // 2 elements. Otherwise, should only have 1 element, unpaired.
            let pairs = edge_map.remove(&edge).expect("Could not find edge");

            // find the other pair
            if let Some(partner_id) = pairs.into_iter().find(|id| *id != tile_id) {
                let partner_tile = tiles_id_map.get(&partner_id).unwrap();
                if let Some(next_image) = get_neighbour(&edge, side, &partner_tile.image) {
                    // if there is a neighbour
                    positions.insert((nx, ny), next_image);
                    queue.push_back((nx, ny, partner_id));
                } else {
                    panic!("Could not find pairing edge for tiles ({} {})", tile_id, partner_id)
                }
            }
        }
    }

    combine_images(&positions)
}


fn combine_images(positions: &HashMap<(i32, i32), Image>) -> Image {
    let (min_x, max_x) = match positions.keys().map(|(x, _)| *x).minmax() {
        MinMaxResult::MinMax(x, y) => (x, y),
        _ => panic!("No keys?!")
    };
    let (min_y, max_y) = match positions.keys().map(|(_, y)| *y).minmax() {
        MinMaxResult::MinMax(x, y) => (x, y),
        _ => panic!("No keys?!")
    };

    let img_size = positions.values().next().unwrap().len() - 2;
    let mut full_image: Image = (min_y..=max_y).flat_map(|_| (0..img_size).map(|_| vec![])).collect();

    for y in (min_y..=max_y).rev() {
        let offset = ((max_y - y) as usize) * img_size;
        for x in min_x..=max_x {
            let img = trim_edges(positions.get(&(x, y)).unwrap());
            for (i, sub_row) in img.into_iter().enumerate() {
                full_image[offset + i].extend(sub_row);
            }
        }
    }

    full_image
}


fn trim_edges(image: &Image) -> Image {
    let stop = image.len() - 1;

    image[1..stop]
        .iter()
        .map(|row| row[1..stop].iter().cloned().collect_vec())
        .collect_vec()
}


fn get_neighbour(image_edge: &str, side: Side, partner_image: &Image) -> Option<Image> {
    let mut pimage = partner_image.clone();
    let mut fimage = flip(&pimage);

    let other_side = match side {
        Side::Top => { Side::Bottom }
        Side::Bottom => { Side::Top }
        Side::Left => { Side::Right }
        Side::Right => { Side::Left }
    };

    for _ in 0..4 {
        if get_image_side(&pimage, other_side) == image_edge {
            return Some(pimage);
        } else if get_image_side(&fimage, other_side) == image_edge {
            return Some(fimage);
        } else {
            pimage = rotate(&pimage);
            fimage = rotate(&fimage);
        }
    }

    None
}

fn rotate(image: &Image) -> Image {
    let n = image.len();
    let mut rotated = image.clone();

    for layer in 0..(n / 2) {
        let limit = n - layer - 1;

        for i in layer..=limit {
            rotated[i][limit] = image[layer][i];
            rotated[limit][i] = image[n - i - 1][limit];
            rotated[i][layer] = image[limit][i];
            rotated[layer][i] = image[n - i - 1][layer];
        }
    }

    rotated
}

fn count_dragons(image: &Image) -> usize {
    let mut count = 0;
    let mut image = image.clone();

    let mut flipped = false;
    let mut rotate_count = 0;
    loop {
        for y in 0..(image.len() - 2) {
            for x in 0..(image[0].len() - 19) {
                let points = get_points(x, y);

                if points.into_iter().all(|(px, py)| image[py][px] == '#') {
                    count += 1;
                }
            }
        }

        if count > 0 {
            return count;
        }

        if rotate_count < 4 {
            rotate_count += 1;
            image = rotate(&image);
        } else if flipped {
            return count;
        } else {
            flipped = true;
            image = flip(&image);
            rotate_count = 0;
        }
    }
}

fn count_rough_waters(image: &Image) -> usize {
    let num_hash: usize = image.iter().map(|v| v.iter().filter(|c| **c == '#').count()).sum();

    let num_dragons = count_dragons(&image);
    num_hash - num_dragons * 15
}

fn get_points(x: usize, y: usize) -> [(usize, usize); 15] {
    [
        (x + 18, y),
        (x, y + 1),
        (x + 5, y + 1),
        (x + 6, y + 1),
        (x + 11, y + 1),
        (x + 12, y + 1),
        (x + 17, y + 1),
        (x + 18, y + 1),
        (x + 19, y + 1),
        (x + 1, y + 2),
        (x + 4, y + 2),
        (x + 7, y + 2),
        (x + 10, y + 2),
        (x + 13, y + 2),
        (x + 16, y + 2),
    ]
}

fn flip(image: &Image) -> Image {
    image.iter()
         .map(|v| v.iter().rev().cloned().collect())
         .collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::d20::{count_dragons, count_rough_waters, find_corner_multiple, Image, parse_input, piece_puzzle, rotate};

    fn test_input<'a>() -> &'a str {
        "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
    }

    #[test]
    fn test_find_corner_multiple() {
        let tiles = parse_input(test_input());
        let ans = find_corner_multiple(tiles);

        assert_eq!(ans, 20899048083289);
    }


    fn image_to_vector(input: &str) -> Image {
        input.lines()
             .map(|line| line.chars().collect())
             .collect()
    }

    #[test]
    fn test_rotate() {
        for (image, expected) in [
            ("
#...#.
..#.##
#..##.
##..##
##...#
.##...", "
.###.#
###...
#...#.
...#..
..####
.##.#."),
            ("
#.#.#.
...###
##..##
###...
.#.#.#
.#####", "
..##.#
####..
#.#..#
##..#.
#..###
##.##."
            )
        ] {
            let rotated = rotate(&image_to_vector(image.trim()));
            let image = rotated.iter()
                               .map(|v| v.iter().collect::<String>())
                               .join("\n");
            assert_eq!(image, expected.trim())
        }
    }

    #[test]
    fn test_piece_puzzle() {
        let tiles = parse_input(test_input());
        let image = piece_puzzle(tiles);

        let num_dragons = count_dragons(&image);
        assert_eq!(num_dragons, 2);

        let num_rough_waters = count_rough_waters(&image);
        assert_eq!(num_rough_waters, 273);
    }
}