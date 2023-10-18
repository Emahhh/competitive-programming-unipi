#![allow(dead_code)]

extern crate core;
use core::cmp::max;
use core::cmp::min;

const PRINT_TREE: bool = true; // set to true to print the tree using the `print_visualization_url()` method






/// Basic Binary Tree implementation
/// # Author
/// - [Professor Venturini](https://pages.di.unipi.it/rossano/blog/2023/handson12324/)
pub mod trees {
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







/// Utility: implementation of print_visualization_url, to visualize the tree in a web browser
mod visualizer {

    use urlencoding::encode;
    use crate::trees;

    impl trees::Tree {

        /// useful to visualize the tree using Graphviz
        /// # Returns 
        /// the DOT representation of the tree,
        pub fn to_dot(&self) -> String {
            let mut dot = String::from("digraph Tree {\n");

            // Traverse the tree and construct the DOT representation
            self.rec_to_dot(0, &mut dot);

            dot.push_str("}\n");
            
            dot
        }

        /// utility method to help print the DOT representation of the tree
        fn rec_to_dot(&self, node_id: usize, dot: &mut String) {
            if let Some(node) = self.get_node(node_id) {
                
                // print the dot line with the information about the current node
                dot.push_str(&format!("  id{} [label=\"id={}\nvalue={}\"];\n", node_id, node_id, node.key));
                
                // print the edge to the left (if it exists) and its subtree
                if let Some(left_id) = node.id_left {
                    dot.push_str(&format!("  id{} -> id{};\n", node_id, left_id));
                    self.rec_to_dot(left_id, dot);
                }
                
                // print the edge to the right (if it exists) and its subtree
                if let Some(right_id) = node.id_right {
                    dot.push_str(&format!("  id{} -> id{};\n", node_id, right_id));
                    self.rec_to_dot(right_id, dot);
                }
            }
        }
        
        pub fn get_visualization_url(&self)-> String {
            let dot_content = self.to_dot();
            let encoded_dot = encode(&dot_content);
            let edotor_url = format!(
                "https://edotor.net/?engine=dot#{}",
                encoded_dot
            );
            edotor_url
        }

        /// prints the URL to visualize the tree
        /// # Arguments
        /// `additional_text`: additional text to print before the URL (useful to recognize the tree between many)
        pub fn print_visualization_url(&self, additional_text: &str) {
            let mut output = String::new();
            output.push_str("=================================\n");
            output.push_str(additional_text);
            output.push_str("\n");
            output.push_str("Put this URL inside a browser to view the tree:\n");
            output.push_str(&self.get_visualization_url());
            output.push_str("\n");
            output.push_str("=================================\n");
            println!("{}", output);
        }
        
    }

}







/// demo to show the print_visualization_url method
fn main() {
    println!("Hello, trees handson!");

    // Create your binary tree
    let mut tree = trees::Tree::with_root(10);
    let _node_b = tree.add_node(0, 5, true);
    let _node_c = tree.add_node(0, 15, false);
    let _node_d = tree.add_node(_node_b, 3, true);
    let _node_e = tree.add_node(_node_b, 7, false);

    tree.print_visualization_url("This is a demo to show the print_visualization_url method.");
}







/// # Exercise 1
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



/// # Tests for exercise 1
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

        if PRINT_TREE {
            ex_tree.print_visualization_url("Tree inside test_is_bst_1:");
        }

        assert!(ex_tree.is_bst() == false , "this should not be a bst!");
    }

    // let's try another tree that is not a bst
    #[test]
    fn test_is_bst_2() {
        let mut t2: trees::Tree = build_example_bst();
        t2.add_node(12, 80, true);

        if PRINT_TREE {
            t2.print_visualization_url("Tree inside test_is_bst_2:");
        }

        assert!(t2.is_bst() == false , "this should not be a bst!");
    }
}










/// # Exercise 2
impl trees::Tree {

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


    pub fn is_balanced(&self) -> bool {
        self.rec_helper_is_balanced(Some(0)).0
    }

    pub fn get_height(&self) -> u32 {
        self.rec_helper_is_balanced(Some(0)).1
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
    fn test_single_node() {
        let tree = Tree::with_root(10);

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_single_node:");
        }

        assert_eq!(tree.is_balanced(), true);
        assert_eq!(tree.get_height(), 1);
    }


    #[test]
    fn test_two_nodes() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 10, true);

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_two_nodes:");
        }

        assert_eq!(tree.is_balanced(), true);
        assert_eq!(tree.get_height(), 2);
    }

    /// this tree should be not balanced, since the left subtree has a height of 2 and the right has 0
    #[test]
    fn test_left_subtree_unbalanced() {
        let mut tree = Tree::with_root(10);
        let last_id = tree.add_node(0, 10, true);
        tree.add_node(last_id, 5, true);

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_left_subtree_unbalanced:");
        }

        assert!( tree.is_balanced() == false);
        assert_eq!(tree.get_height(), 3);
    }

    #[test]
    fn test_right_subtree_unbalanced() {
        let mut tree = Tree::with_root(10);
        let last_id = tree.add_node(0, 10, false);
        tree.add_node(last_id, 5, false);

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_right_subtree_unbalanced:");
        }

        assert!(tree.is_balanced() == false);
        assert_eq!(tree.get_height(), 3);
    }

    #[test]
    fn test_simple_balanced_tree() {
        let mut tree = Tree::with_root(10);
        let _left_id = tree.add_node(0, 10, true);
        let _right_id = tree.add_node(0, 10, false);

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_simple_balanced_tree:");
        }

        assert!(tree.is_balanced() == true);
        assert_eq!(tree.get_height(), 2);
    }


    #[test]
    fn test_another_balanced_tree() {
        let mut tree = Tree::with_root(10);
        let _left_id = tree.add_node(0, 10, true);
        let _right_id = tree.add_node(0, 10, false);

        tree.add_node(_left_id, 999, true);
        tree.add_node(_right_id, 65, false);

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_another_balanced_tree:");
        }

        assert!(tree.is_balanced() == true);
        assert_eq!(tree.get_height(), 3);
    }


}







/// # Exercise 3
impl trees::Tree {
    // Write a method to check if the binary tree is a max-heap.
    // A max-heap is a complete binary tree in which every node satisfies the max-heap property.
    // Complete Binary Tree: Every level in the binary tree, except possibly the last/lowest level, is completely filled, and all vertices in the last level are as far left as possible.
    // A node satisfies the max-heap property: if its key is greater than or equal to the keys of its children.

    /// # Returns
    /// true iff the tree is a max-heap
    pub fn is_max_heap(&self) -> bool {
        self.rec_helper_heap(Some(0), 0, self.get_height(), 0).0
    }


    /// # Returns
    /// a couple, where:
    /// - the first item is true iff the tree is a max-heap
    /// - the second item is the max value in the tree
    pub fn is_max_heap_with_max(&self) -> (bool, u32) {
        let tree_height = self.get_height();

        self.rec_helper_heap(Some(0), 0, tree_height, 0)
    }




    fn rec_helper_heap(&self, curr_id_opt: Option<usize>, curr_level: u32, tree_height: u32, incomplete_nodes_count: u32) -> (bool, u32){
        if curr_id_opt.is_none(){
            // in a complete tree, only the two lowest level can have empty nodes
            if (curr_level == tree_height) ||  (curr_level == tree_height-1) { 
                return (true, curr_level); 
            }
            return (false, curr_level); 
        }
        let curr_id = curr_id_opt.unwrap();


        let curr_node_opt = &self.get_node(curr_id);
        if curr_node_opt.is_none() {
            if (curr_level == tree_height) ||  (curr_level == tree_height-1) {
                return (true, curr_level); 
            }
            return (false, curr_level); 
        }
        let curr_node = curr_node_opt.unwrap();

        // complete tree property
        // let's check if the current tree is a complete binary tree
        let mut is_complete: bool = true;

        if curr_node.id_left.is_some() && curr_node.id_right.is_none(){ // if this node has a left child but not a right child
            // TODO: replace with "i have seen right incomplete node before" flag
            let incomplete_nodes_count_new = incomplete_nodes_count + 1;
            if incomplete_nodes_count_new > 1 {
                is_complete = false;
            }
        }

        let (is_max_heap_left, max_left) = self.rec_helper_heap(curr_node.id_left, curr_level+1, tree_height, incomplete_nodes_count);
        let (is_max_heap_right, max_right) = self.rec_helper_heap(curr_node.id_right, curr_level+1, tree_height, incomplete_nodes_count);






        // let's check the max-heap property for the current node
        // the current node must have a value greater than its left and right subtrees
        let curr_max = max(curr_node.key, max(max_left, max_right)   );
        let is_max: bool = curr_node.key == curr_max;

        // finally, we put the conditions together
        let is_max_heap = is_max_heap_left && is_max_heap_right && is_complete && is_max;
        return (is_max_heap, curr_max );
    }


/*     pub fn get_height(&self) -> u32 {
        self.rechelper_get_height(Some(0), 0) 
    }

    fn rechelper_get_height(&self, curr_id_opt: Option<usize>, curr_level: u32) -> u32{
        if curr_id_opt.is_none(){
            return curr_level;
        }
        let curr_id = curr_id_opt.unwrap();

        let curr_node_opt = &self.get_node(curr_id);
        if curr_node_opt.is_none() {
            return curr_level;
        }

        let curr_node = curr_node_opt.unwrap();

        return max( self.rechelper_get_height(curr_node.id_left, curr_level + 1), self.rechelper_get_height(curr_node.id_right, curr_level + 1) );
    } */

}









#[cfg(test)]
mod max_heap_tests {
    use super::*;
    use trees::*;

    #[test]
    fn test_single_node_max_heap() {
        let tree = Tree::with_root(10);
        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_single_node_max_heap:");
        }

        assert_eq!(is_max_heap, true);
        assert_eq!(max_value, 10);
    }

    #[test]
    fn test_two_nodes_max_heap() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 9, true);
        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        if PRINT_TREE {
            tree.print_visualization_url("Tree inside test_two_nodes_max_heap:");
        }

        assert_eq!(is_max_heap, true);
        assert_eq!(max_value, 10);
    }

    #[test]
    fn test_max_heap_with_left_subtree_unbalanced() {
        let mut tree = Tree::with_root(10);
        let last_id = tree.add_node(0, 10, true);
        tree.add_node(last_id, 5, true);
        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        let mut output = String::new();
        output.push_str("Function: test_max_heap_with_left_subtree_unbalanced\n");
        output.push_str("==================================================\n");
        output.push_str("Visualization URL: ");
        output.push_str(&tree.get_visualization_url());
        output.push_str("\n------------------------------\n");

        println!("{}", output);

        assert_eq!(is_max_heap, false);
        assert_eq!(max_value, 10);
    }

    #[test]
    fn test_max_heap_with_right_subtree_unbalanced() {
        let mut tree = Tree::with_root(10);
        let last_id = tree.add_node(0, 10, false);
        tree.add_node(last_id, 5, false);
        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        let mut output = String::new();
        output.push_str("Function: test_max_heap_with_right_subtree_unbalanced\n");
        output.push_str("===================================================\n");
        output.push_str("Visualization URL: ");
        output.push_str(&tree.get_visualization_url());
        output.push_str("\n------------------------------\n");

        println!("{}", output);

        assert_eq!(is_max_heap, false);
        assert_eq!(max_value, 10);
    }


    #[test]
    fn test_max_heap_small() {
        let mut tree = Tree::with_root(9);

        tree.add_node(0, 5, true);
        tree.add_node(0, 9, false);

        tree.add_node(1, 4, false);

        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        let mut output = String::new();
        output.push_str("Function: test_max_heap_small\n");
        output.push_str("===================================================\n");
        output.push_str("Visualization URL: ");
        output.push_str(&tree.get_visualization_url());
        output.push_str("\n------------------------------\n");

        println!("{}", output);

        assert_eq!(is_max_heap, true);
        assert_eq!(max_value, 9);
    }


    /// generated using https://visualgo.net/en/heap
    #[test]
    fn a_random_heap() {
        let mut tree = Tree::with_root(76);

        tree.add_node(0, 67, true); // id 1
        tree.add_node(0, 36, false); // id 2

        tree.add_node(1, 62, true); // id 3
        tree.add_node(1, 62, false); // id 4
        tree.add_node(2, 32, true); // id 5
        tree.add_node(2, 15, false); // id 6

        tree.add_node(3, 5, false); // id 7
        tree.add_node(3, 15, true); // id 8
        tree.add_node(4, 57, true); // id 9

        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        let mut output = String::new();
        output.push_str("Function: a_random_heap\n");
        output.push_str("===================================================\n");
        output.push_str("Visualization URL: ");
        output.push_str(&tree.get_visualization_url());
        output.push_str("\n------------------------------\n");

        println!("{}", output);

        assert_eq!(is_max_heap, true);
        assert_eq!(max_value, 76);
    }


    /// same as a_random_heap, but with a changed value
    #[test]
    fn a_random_non_heap() {
        let mut tree = Tree::with_root(76);

        tree.add_node(0, 67, true); // id 1
        tree.add_node(0, 36, false); // id 2

        tree.add_node(1, 62, true); // id 3
        tree.add_node(1, 62, false); // id 4
        tree.add_node(2, 32, true); // id 5
        tree.add_node(2, 15, false); // id 6

        tree.add_node(3, 5, false); // id 7
        tree.add_node(3, 64, true); // id 8
        tree.add_node(4, 57, true); // id 9

        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        let mut output = String::new();
        output.push_str("Function: a_random_non_heap\n");
        output.push_str("===================================================\n");
        output.push_str("Visualization URL: ");
        output.push_str(&tree.get_visualization_url());
        output.push_str("\n------------------------------\n");

        println!("{}", output);

        assert_eq!(is_max_heap, false);
        assert_eq!(max_value, 76);
    }

    #[test]
    fn another_random_non_heap() {
        let mut tree = Tree::with_root(76);

        tree.add_node(0, 67, true); // id 1
        tree.add_node(0, 36, false); // id 2

        tree.add_node(1, 62, true); // id 3
        tree.add_node(1, 62, false); // id 4
        tree.add_node(2, 37, true); // id 5
        tree.add_node(2, 15, false); // id 6

        tree.add_node(3, 5, false); // id 7
        tree.add_node(3, 15, true); // id 8
        tree.add_node(4, 57, true); // id 9

        let (is_max_heap, max_value) = tree.is_max_heap_with_max();

        let mut output = String::new();
        output.push_str("Function: a_random_non_heap\n");
        output.push_str("===================================================\n");
        output.push_str("Visualization URL: ");
        output.push_str(&tree.get_visualization_url());
        output.push_str("\n------------------------------\n");
        println!("{}", output);

        assert_eq!(is_max_heap, false);
        assert_eq!(max_value, 76);
    }




}