#![feature(slice_patterns)]

use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Hash, Default)]
struct Tree {
    nodes: [Option<Box<Tree>>; 2],
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Tree { nodes: [self.nodes[0].clone(), self.nodes[1].clone()] }
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        (match (&self.nodes[0], &other.nodes[0]) {
             (&Some(ref a), &Some(ref b)) => a == b,
             (&None, &None) => true,
             _ => false,
         }) &&
        match (&self.nodes[1], &other.nodes[1]) {
            (&Some(ref a), &Some(ref b)) => a == b,
            (&None, &None) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl Eq for Tree {}

impl Tree {
    fn depth(&self) -> usize {
        self.nodes[0]
            .as_ref()
            .map(|n| n.depth())
            .and_then(|n| self.nodes[1].as_ref().map(|m| max(m.depth(), n)))
            .unwrap_or(0) + 1
    }

    /// This only works when a unique_map of every unique tree which is a depth below this tree has been completed.
    fn unique_isomorphic(&self, unique_map: &HashMap<Tree, usize>) -> Self {
        match self.nodes {
            [Some(ref a), Some(ref b)] => {
                let a = a.clone();
                let b = b.clone();
                let a = a.unique_isomorphic(unique_map);
                let b = b.unique_isomorphic(unique_map);
                let a_id =
                    unique_map
                        .get(&a)
                        .unwrap_or_else(|| panic!("error: failed to get unique id of {:?}", a));
                let b_id =
                    unique_map
                        .get(&b)
                        .unwrap_or_else(|| panic!("error: failed to get unique id of {:?}", b));

                if a_id < b_id {
                    Tree { nodes: [Some(Box::new(a)), Some(Box::new(b))] }
                } else {
                    Tree { nodes: [Some(Box::new(b)), Some(Box::new(a))] }
                }
            }
            // If the second is something, its longer.
            [None, Some(ref b)] => Tree { nodes: [Some(Box::new(b.unique_isomorphic(unique_map))), None] },
            [Some(ref a), None] => Tree { nodes: [Some(Box::new(a.unique_isomorphic(unique_map))), None] },
            [None, None] => Tree { nodes: [None, None] },
        }
    }

    /// Increment the binary tree to the next iteration and return the carry.
    /// The `depth` represents the maximum depth that can be reached on the left before the right must get incremented.
    fn inc(&mut self, depth: usize) -> bool {
        let mut right_try_carry = false;
        let mut right_create = false;
        let mut left_create = false;
        if depth == 0 {
            return true;
        } else {
            match self.nodes {
                [Some(ref mut l), Some(_)] => {
                    if l.inc(depth - 1) {
                        right_try_carry = true;
                    }
                }
                [Some(ref mut l), None] => {
                    if l.inc(depth - 1) {
                        right_create = true;
                    }
                }
                [None, None] => {
                    left_create = true;
                }
                _ => panic!("error: found improper sub-tree {:?}", self),
            }
        }
        if right_create {
            self.nodes[1] = Some(Default::default());
            false
        } else if left_create {
            self.nodes[0] = Some(Default::default());
            false
        } else if right_try_carry {
            if self.nodes[1].as_mut().unwrap().inc(depth - 1) {
                self.nodes[1] = None;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Prints the tree in simple form.
    fn print_simple(&self) {
        print!("{{");
        if let Some(ref n) = self.nodes[0] {
            n.print_simple();
        }
        print!(",");
        if let Some(ref n) = self.nodes[1] {
            n.print_simple();
        }
        print!("}}");
    }

    /// Prints the tree in fancy form.
    fn print_fancy(&self, depth: usize) {
        for _ in 0..depth {
            print!("\t");
        }
        if self.nodes.iter().all(Option::is_none) {
            println!("*");
            return;
        }
        println!("1->");
        if let Some(ref n) = self.nodes[0] {
            n.print_fancy(depth + 1);
        }
        if let Some(ref n) = self.nodes[1] {
            for _ in 0..depth {
                print!("\t");
            }
            println!("2->");
            n.print_fancy(depth + 1);
        }
    }
}

#[derive(Debug)]
struct TreeIterator {
    current: Tree,
    depth: usize,
}

impl Iterator for TreeIterator {
    type Item = Tree;

    fn next(&mut self) -> Option<Tree> {
        let out = self.current.clone();
        if self.current.inc(self.depth) {
            self.depth += 1;
            self.current.inc(self.depth);
        }
        Some(out)
    }
}

impl TreeIterator {
    fn new() -> TreeIterator {
        TreeIterator {
            current: Tree::default(),
            depth: 0,
        }
    }
}

#[derive(Debug)]
struct UniqueTreeIterator {
    unique_map: HashMap<Tree, usize>,
    tree_iter: TreeIterator,
}

impl Iterator for UniqueTreeIterator {
    type Item = Tree;

    fn next(&mut self) -> Option<Tree> {
        let uid = self.unique_map.len();
        let unique_map = &mut self.unique_map;
        self.tree_iter
            .find(|t| {
                let t = t.unique_isomorphic(unique_map);
                unique_map.insert(t.clone(), uid).is_none()
            })
    }
}

impl UniqueTreeIterator {
    fn new() -> UniqueTreeIterator {
        UniqueTreeIterator {
            unique_map: HashMap::new(),
            tree_iter: TreeIterator::new(),
        }
    }
}

fn main() {
    for (ix, tree) in UniqueTreeIterator::new().take(100).enumerate() {
        println!("{}: ", ix);
        tree.print_fancy(0);
        println!();
    }
}

