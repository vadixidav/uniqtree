#![feature(slice_patterns)]

use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Hash)]
struct Tree {
    nodes: [Option<Box<Tree>>; 2],
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Tree { nodes: [self.nodes[0].clone(), self.nodes[0].clone()] }
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
}

fn main() {
    
}

