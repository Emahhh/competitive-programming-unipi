/// Basic Binary Tree implementation
///
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
        nodes: Vec<Node>,
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

impl trees::Tree {
    /// Exercise 1
    /// Checks if the binary tree is a Binary Search Tree.
    pub fn is_bst(&self) -> bool {
        self.helper_rec_is_bst(Some(0)).0
    }

    fn helper_rec_is_bst(&self, curr_root_id: Option<usize>) -> (bool, u32, u32) {
        const NEG_INF: u32 = u32::MIN; // since we are dealing with u32, the value is actually 0
        const INF: u32 = u32::MAX;

        if curr_root_id.is_none() {
            return (true, NEG_INF, INF);
        }

        let current_root_opt: Option<&trees::Node> = self.get_node(curr_root_id.unwrap());
        if current_root_opt.is_none() {
            return (false, NEG_INF, INF);
        }

        let root: &trees::Node = current_root_opt.unwrap();

        let (is_left_bst, max_left, _ ) = self.helper_rec_is_bst(root.id_left);
        let (is_right_bst, _ , min_right) = self.helper_rec_is_bst(root.id_right);

        if !is_left_bst || !is_right_bst {
            return (false, NEG_INF, INF);
        }

        let am_i_bst: bool = max_left <= root.key && root.key <= min_right;

        let my_max = core::cmp::max(core::cmp::max(root.key, max_left), min_right);
        let my_min = core::cmp::min(core::cmp::min(root.key, max_left), min_right);
        return (am_i_bst, my_max, my_min);
        
        
    }



}

pub fn main() {
    println!("Hello, trees handson!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bst_1() {
        let mut tree = trees::Tree::with_root(10);
        assert!(tree.is_bst());

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert!(tree.is_bst());
    }
}
