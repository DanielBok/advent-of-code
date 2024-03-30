use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

use itertools::Itertools;

use crate::inputs::read_contents;

#[derive(Clone)]
enum Tree {
    Leaf { parent: TreePtr, value: usize },
    Node { parent: TreePtr, left: TreePtr, right: TreePtr },
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Leaf { value, .. } => {
                write!(f, "{}", value)
            }
            Tree::Node { left, right, .. } => {
                write!(f, "[{:?},{:?}]", **left, **right)
            }
        }
    }
}

impl Tree {
    fn set_parent(&mut self, parent: &TreePtr) {
        match self {
            Tree::Leaf { parent: this_parent, .. } => {
                *this_parent = (*parent).clone();
            }
            Tree::Node { parent: this_parent, .. } => {
                *this_parent = (*parent).clone();
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct TreePtr(*mut Tree);

impl From<Tree> for TreePtr {
    fn from(value: Tree) -> Self {
        Self(Box::into_raw(Box::from(value)))
    }
}

impl From<&str> for TreePtr {
    fn from(input: &str) -> Self {
        Self::from(&*input.chars().collect_vec())
    }
}


impl From<&[char]> for TreePtr {
    fn from(input: &[char]) -> Self {
        fn get_split_index(input: &[char]) -> usize {
            let mut open_brackets = 0;

            for (i, c) in input.iter().enumerate() {
                match *c {
                    '[' => {
                        open_brackets += 1;
                    }
                    ']' => {
                        open_brackets -= 1;
                    }
                    _ => {}
                }

                if i > 0 && open_brackets == 1 {
                    return i + 1;
                }
            }

            panic!("This should not happen")
        }

        let split_index = get_split_index(input);

        let (left, right) = input.split_at(split_index);
        let left = &left[1..left.len()];  // drop first open bracket and the split comma
        let right = &right[1..right.len() - 1];  // drop last close bracket

        let left = if left[0].is_digit(10) {
            let value = left[0].to_digit(10).unwrap() as usize;
            TreePtr::from(Tree::Leaf { parent: TreePtr::null(), value })
        } else {
            TreePtr::from(left)
        };

        let right = if right[right.len() - 1].is_digit(10) {
            let value = right[right.len() - 1].to_digit(10).unwrap() as usize;
            TreePtr::from(Tree::Leaf { parent: TreePtr::null(), value })
        } else {
            TreePtr::from(right)
        };

        TreePtr::combine(left, right)
    }
}

impl From<Vec<&str>> for TreePtr {
    fn from(input: Vec<&str>) -> Self {
        let mut iter = input.into_iter();
        let mut ptr = TreePtr::from(iter.next().unwrap());

        for line in iter {
            ptr = TreePtr::combine(ptr, TreePtr::from(line));
            ptr.operate();
        }

        ptr
    }
}


impl Deref for TreePtr {
    type Target = Tree;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0) }
    }
}

impl DerefMut for TreePtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.0) }
    }
}

impl Debug for TreePtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{:?}", self.deref())
    }
}

enum Side {
    Left,
    Right,
}

impl TreePtr {
    fn is_null(&self) -> bool {
        self.0.is_null()
    }

    fn null() -> Self {
        Self(null_mut())
    }

    fn operate(&mut self) {
        let mut explode_only = true;
        loop {
            match self.operate_at(0, explode_only) {
                Operation::Stop => {
                    explode_only = true;
                }
                Operation::Noop => {
                    if explode_only {
                        explode_only = false;
                    } else {
                        return;
                    }
                }
            }
        }
    }

    fn operate_at(&mut self, level: usize, explode_only: bool) -> Operation {
        let addr = self.0;

        match self.deref_mut() {
            Tree::Leaf { value, parent } => {
                if !explode_only && *value > 9 {
                    // split ops
                    let lv = *value / 2;
                    let rv = lv + *value % 2;

                    let mut split = TreePtr::combine(
                        TreePtr::from(Tree::Leaf { value: lv, parent: Self::null() }),
                        TreePtr::from(Tree::Leaf { value: rv, parent: Self::null() }),
                    );
                    split.set_parent(parent);
                    *self = split;

                    // split inplace and stop all operations
                    return Operation::Stop;
                }
                return Operation::Noop;
            }
            Tree::Node { left, right, parent } => {
                if level >= 4 {
                    let lv = match left.deref().deref() {
                        Tree::Leaf { value, .. } => *value,
                        _ => panic!("Explosion, expected left to be a Regular but got: {:?}", left.deref().deref())
                    };

                    let rv = match right.deref().deref() {
                        Tree::Leaf { value, .. } => *value,
                        _ => panic!("Explosion, expected right to be a Regular but got: {:?}", right.deref().deref())
                    };

                    parent.explode(Side::Left, lv, addr);
                    parent.explode(Side::Right, rv, addr);


                    *self = TreePtr::from(
                        Tree::Leaf { value: 0, parent: parent.clone() }
                    );

                    return Operation::Stop;
                }

                match left.operate_at(level + 1, explode_only) {
                    Operation::Stop => return Operation::Stop,
                    Operation::Noop => {}  // do nothing
                }

                match right.operate_at(level + 1, explode_only) {
                    Operation::Stop => return Operation::Stop,
                    Operation::Noop => {}  // do nothing
                }

                return Operation::Noop;
            }
        };
    }

    fn explode(&self, side: Side, delta: usize, current: *mut Tree) {
        // all comments assume exploding left. Exploding right works in a vice versa manner
        match self.deref() {
            Tree::Leaf { .. } => {
                panic!("Explosion: parent cannot be a leaf!")
            }
            Tree::Node { left, right, parent } => {
                let comparison = if matches!(side, Side::Left) { left } else { right };
                if comparison.0 == current {
                    // if parent is null, then we reached the root node. We just ignore any
                    // more operations
                    if parent.is_null() {
                        return;
                    }

                    // we keep going up the tree until we reach a point where the current node
                    // is on the right. At this point, we have a place where we can add the
                    // delta to a child on the left node
                    parent.explode(side, delta, self.0);
                } else {
                    let mut curr = &mut (*comparison).clone();  // re-borrow
                    // if we are exploding left, then the current node is the right node

                    loop {
                        match curr.deref_mut() {
                            Tree::Leaf { value, .. } => {
                                // when we reach a node that is a leaf node, we add the delta
                                *value += delta;
                                return;
                            }
                            Tree::Node { left: next_left, right: next_right, .. } => {
                                // otherwise, we need to keep searching for the right most node
                                // by searching we are going down the right side from the left
                                // ancestor
                                curr = if matches!(side, Side::Left) { next_right } else { next_left };
                            }
                        }
                    }
                }
            }
        }
    }

    fn combine(left: TreePtr, right: TreePtr) -> TreePtr {
        let mut this = TreePtr::from(
            Tree::Node {
                parent: TreePtr::null(),
                left: TreePtr::null(),
                right: TreePtr::null(),
            }
        );
        let mut left_ptr = left;
        left_ptr.set_parent(&this);

        let mut right_ptr = right;
        right_ptr.set_parent(&this);

        match this.deref_mut() {
            Tree::Node { left, right, .. } => {
                *left = left_ptr;
                *right = right_ptr;
            }
            _ => {}
        }

        this
    }

    fn magnitude(&self) -> usize {
        match self.deref() {
            Tree::Leaf { value, .. } => *value,
            Tree::Node { left, right, .. } => {
                3 * left.magnitude() + 2 * right.magnitude()
            }
        }
    }
}


enum Operation {
    Stop,
    Noop,
}

pub fn solve_a() {
    let tree_ptr = TreePtr::from(read_contents(18).lines().collect_vec());

    println!("Solution A: {}", tree_ptr.magnitude());
}

pub fn solve_b() {
    let input = read_contents(18);
    let lines = input.lines().collect_vec();

    let ans = get_largest_magnitude(lines);
    println!("Solution B: {}", ans);
}

fn get_largest_magnitude(lines: Vec<&str>) -> usize {
    let mut max = 0;

    for (i, line1) in lines.iter().enumerate() {
        for (j, line2) in lines.iter().enumerate() {
            if i == j { continue; }
            let mut ptr = TreePtr::combine(TreePtr::from(*line1), TreePtr::from(*line2));
            ptr.operate();
            let magnitude = ptr.magnitude();

            if magnitude > max {
                max = magnitude;
            }
        }
    }

    max
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use itertools::Itertools;

    use crate::d18::{get_largest_magnitude, Tree, TreePtr};

    impl TreePtr {
        fn to_string(&self) -> String {
            match self.deref() {
                Tree::Leaf { value, .. } => {
                    format!("{}", value)
                }
                Tree::Node { left, right, .. } => {
                    format!("[{},{}]", left.to_string(), right.to_string())
                }
            }
        }
    }

    #[test]
    fn test_basic_operations() {
        for (input, exp) in [
            ("[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
            ("[1,1]
[2,2]
[3,3]
[4,4]", "[[[[1,1],[2,2]],[3,3]],[4,4]]"),
            ("[1,1]
[2,2]
[3,3]
[4,4]
[5,5]", "[[[[3,0],[5,3]],[4,4]],[5,5]]"),
            ("[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]", "[[[[5,0],[7,4]],[5,5]],[6,6]]"),
            ("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        ] {
            let input = input.lines().collect::<Vec<_>>();
            let mut ptr = TreePtr::from(input);
            ptr.operate();
            assert_eq!(ptr.to_string(), exp);
        }
    }

    #[test]
    fn test_magnitude() {
        for (input, exp) in [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ] {
            let tree_ptr = TreePtr::from(input);
            assert_eq!(tree_ptr.magnitude(), exp, "{:?}", tree_ptr);
        }
    }

    #[test]
    fn test_get_largest_magnitude() {
        let lines = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".lines().collect_vec();

        let ans = get_largest_magnitude(lines);
        assert_eq!(ans, 3993);
    }
}