struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn tree_size(n: Option<Box<TreeNode>>) -> i32 {
    if n.is_none() {
        return 0;
    }

    let node = n.unwrap();
    let size_left = tree_size(node.left);
    let size_right = tree_size(node.right);

    1 + size_left + size_right
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_tree_size_empty() {
        let n: Option<Box<TreeNode>> = None;
        let result = tree_size(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_tree_size_single_node() {
        let n = Some(Box::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        }));
        let result = tree_size(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_tree_size_complex_tree() {
        let n = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        }));
        let result = tree_size(n);
        assert_eq!(result, 4);
    }

}



pub fn main() {
    println!("Hello, trees!");
}