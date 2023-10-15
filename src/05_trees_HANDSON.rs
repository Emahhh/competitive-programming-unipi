#![allow(dead_code)]

extern crate core;
use core::cmp::max;
use core::cmp::min;

pub fn main() {
    println!("Hello, trees handson!");
}


/// Basic Binary Tree implementation
/// # Author
/// - [Professor Venturini](https://pages.di.unipi.it/rossano/blog/2023/handson12324/)
#[allow(dead_code)]
mod trees {
    pub struct Node {
        pub key: u32,
        pub id_left: Option<usize>,
        pub id_right: Option<usize>,
    }

    impl Node {
        fn new(key: u32) -> Self {
            Self {
                key,
                id_left: None,
                id_right: None,
            }
        }
    }

    pub struct Tree {
        pub nodes: Vec<Node>,
    }

    /// This a representation of a tree.
    /// Every node has an implicity id, which is its position on the vector `nodes`.
    /// Every node has a key and at most two children. The ids of the children are
    /// stored in `id_left` and `id_right`. These ids are `None` iff the child does not exit.
    impl Tree {
        pub fn with_root(key: u32) -> Self {
            Self {
                nodes: vec![Node::new(key)],
            }
        }

        pub fn get_node(&self, id: usize) -> Option<&Node> {
            self.nodes.get(id)
        }

        /// Adds a child to the node with `parent_id` and returns the id of the new node.
        /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
        /// iff `is_left` is `true`, the right child otherwise.
        ///
        /// # Panics
        /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
        pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
            assert!(
                parent_id < self.nodes.len(),
                "Parent node id does not exist"
            );
            if is_left {
                assert!(
                    self.nodes[parent_id].id_left.is_none(),
                    "Parent node has the child already set"
                );
            } else {
                assert!(
                    self.nodes[parent_id].id_right.is_none(),
                    "Parent node has the right child already set"
                );
            }

            let child_id = self.nodes.len();
            self.nodes.push(Node::new(key));

            let child = if is_left {
                &mut self.nodes[parent_id].id_left
            } else {
                &mut self.nodes[parent_id].id_right
            };

            *child = Some(child_id);

            child_id
        }

        /// Returns the sum of all the keys in the tree
        pub fn sum(&self) -> u32 {
            self.rec_sum(Some(0))
        }

        /// A private recursive function that computes the sum of
        /// nodes in the subtree rooted at `node_id`.
        fn rec_sum(&self, node_id: Option<usize>) -> u32 {
            if let Some(id) = node_id {
                assert!(id < self.nodes.len(), "Node id is out of range");
                let node = &self.nodes[id];

                let sum_left = self.rec_sum(node.id_left);
                let sum_right = self.rec_sum(node.id_right);

                return sum_left + sum_right + node.key;
            }

            0
        }
    }
}





// Start of EXERCISE 1 ------------------------------------------------------------------------------------

impl trees::Tree {


    /// # Returns
    /// True iff the tree (rooted at id 0) is a binary search tree. False otherwise.
    pub fn is_bst(&self) -> bool {
        self.helper_rec_is_bst(Some(0)).0
    }

    /// Helper recursive function for is_bst
    /// # Returns
    /// A tuple `(is_bst, max, min)`,
    /// where `max` is the maximum value in the subtree, and `min` is the minimum.
    fn helper_rec_is_bst(&self, curr_root_id: Option<usize>) -> (bool, u32, u32) {
        const NEG_INF: u32 = u32::MIN; // since we are dealing with u32, the value is actually 0
        const INF: u32 = u32::MAX;

        if curr_root_id.is_none() {
            return (true, NEG_INF, INF);
        }

        let root: &trees::Node = self.get_node(curr_root_id.unwrap()).unwrap(); // should always be Some(node), unless the tree is not valid

        let (is_left_bst, max_left, min_left ) = self.helper_rec_is_bst(root.id_left);
        let (is_right_bst, max_right, min_right) = self.helper_rec_is_bst(root.id_right);

        if !is_left_bst || !is_right_bst {
            return (false, NEG_INF, INF);
        }

        let am_i_bst: bool = max_left <= root.key && root.key <= min_right;

        let new_max = max(root.key, max(max_left, max_right));
        let new_min = min(root.key, min(min_left, min_right));
        
        return (am_i_bst, new_max, new_min);
        
    }



}


/// Tests for exercise 1
#[cfg(test)]
mod ex_1_tests {
    use super::*;

    fn build_example_bst() -> trees::Tree {
        let mut tree = trees::Tree::with_root(20);

        // first level
        tree.add_node(0, 10, true); // id 1
        tree.add_node(0, 21, false); // id 2

        // second level
        tree.add_node(1, 6, true); // id 3
        tree.add_node(1, 15, false); // id 4

        tree.add_node(2, 20, true); // id 5
        tree.add_node(2, 100, false); //id 6

        // third level
        tree.add_node(3,1, true); // id 7
        tree.add_node(3, 8, false); // id 8

        tree.add_node(4, 14, true); // id 9
        tree.add_node(4, 18, false); // id 10

        tree.add_node(6, 115, false); // id 11

        // fourth level
        tree.add_node(11, 115, false); // id 12
        tree.add_node(11, 115, true);


        return tree;
    }

    #[test]
    fn test_is_bst_1() {
        // let's check if this tree is recognized as a bst
        let mut ex_tree = build_example_bst();
        assert!(ex_tree.is_bst());

        // let's add a node to this tree so that it is not a bst anymore
        ex_tree.add_node(12, 13, false);
        assert!(ex_tree.is_bst() == false , "this should not be a bst!");
    }

    // let's try another tree that is not a bst
    #[test]
    fn test_is_bst_2() {
        let mut t2: trees::Tree = build_example_bst();
        t2.add_node(12, 80, true);
        assert!(t2.is_bst() == false , "this should not be a bst!");
    }
}

// End of EXERCISE 1 ------------------------------------------------------------------------------------






// Start of EXERCISE 2 ------------------------------------------------------------------------------------

/*
# Request:
Write a method to check if the binary tree is balanced.
A tree is considered balanced if, for each of its nodes, the heights of its left and right subtrees differ by at most one.

# Pseudocode:

```
is_balanced(node) -> (bool, int) {
    if node is None {
        return (True, 0)
    }

    let (is_balanced_left, height_left) = is_balanced(node.left);
    let (is_balanced_right, height_right) = is_balanced(node.right);

    let am_i_balanced = is_balanced_left && is_balanced_right && abs(height_left - height_right) <= 1;
    let new_height = max(height_left, height_right) + 1;

    return (am_i_balanced, new_height);
}
```

*/


impl trees::Tree {
    pub fn is_balanced(&self) -> bool {
        self.rec_helper_is_balanced(Some(0)).0
    }

    /// Recursively checks if the binary tree is balanced.
    ///
    /// This function is used  (as a helper function) to recursively determine if the binary tree, rooted at the `curr_id_opt` node,
    /// is balanced.
    ///
    /// # Arguments
    ///
    /// * `curr_id_opt` - The ID of the node to be considered as the root. Pass `None` to check the entire tree.
    ///
    /// # Returns
    ///
    /// A tuple `(bool, u32)` where the first element is `true` if the tree is balanced, and the second element
    /// represents the height of the tree.
    fn rec_helper_is_balanced(&self, curr_id_opt: Option<usize>) -> (bool, u32) {
        if curr_id_opt.is_none(){
            return (true, 0);
        }
        let curr_id = curr_id_opt.unwrap();


        let curr_node_opt = self.get_node(curr_id);
        if curr_node_opt.is_none() {
            return (true, 0);
        }
        let curr_node = curr_node_opt.unwrap();

        let (is_balanced_left, height_left) = self.rec_helper_is_balanced(curr_node.id_left);
        let (is_balanced_right, height_right) = self.rec_helper_is_balanced(curr_node.id_right);

        let am_i_balanced: bool = 
            is_balanced_left &&
            is_balanced_right &&
            ( height_left.abs_diff(height_right) <= 1 )
        ;

        let new_height = max(height_left, height_right) + 1;

        return (am_i_balanced, new_height);
    }



}



/// Tests for exercise 2
#[cfg(test)]
mod ex_2_tests {
    use super::*;
    use trees::*;

    #[test]
    fn test_empty_tree() {
        let tree = Tree::with_root(10);
        let (is_balanced, height) = tree.rec_helper_is_balanced(None);

        assert_eq!(is_balanced, true);
        assert_eq!(height, 0);
    }

    #[test]
    fn test_single_node() {
        let mut tree = Tree::with_root(10);
        let root_id = tree.add_node(0, 10, false);

        let (is_balanced, height) = tree.rec_helper_is_balanced(Some(root_id));

        assert_eq!(is_balanced, true);
        assert_eq!(height, 1);
    }

    /// this tree should be not balanced, since the left subtree has a height of 2 and the right has 0
    #[test]
    fn test_left_subtree_unbalanced() {
        let mut tree = Tree::with_root(10);
        let mut last_id = tree.add_node(0, 10, true);
        tree.add_node(last_id, 5, true);

        let (is_balanced, height) = tree.rec_helper_is_balanced(Some(0));

        assert!( !is_balanced );
        assert_eq!(height, 3);
    }

    #[test]
    fn test_right_subtree_unbalanced() {
        let mut tree = Tree::with_root(10);
        let mut last_id = tree.add_node(0, 10, false);
        tree.add_node(last_id, 5, false);

        let (is_balanced, height) = tree.rec_helper_is_balanced(Some(0));

        assert!(!is_balanced);
        assert_eq!(height, 3);
    }

    #[test]
    fn test_simple_balanced_tree() {
        let mut tree = Tree::with_root(10);
        let _left_id = tree.add_node(0, 10, true);
        let _right_id = tree.add_node(0, 10, false);

        let (is_balanced, height) = tree.rec_helper_is_balanced(Some(0));

        assert!(is_balanced);
        assert_eq!(height, 2);
    }

    #[test]
    fn test_another_balanced_tree() {
        let mut tree = Tree::with_root(10);
        let _left_id = tree.add_node(0, 10, true);
        let _right_id = tree.add_node(0, 10, false);

        let (is_balanced, height) = tree.rec_helper_is_balanced(Some(0));

        assert!(is_balanced);
        assert_eq!(height, 2);
    }
}


// End of EXERCISE 2 ------------------------------------------------------------------------------------