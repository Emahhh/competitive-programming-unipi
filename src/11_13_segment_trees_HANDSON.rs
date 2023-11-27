#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::bool_assert_comparison)]

// https://pages.di.unipi.it/rossano/blog/2023/handson22324/


pub mod ex_1_segment {



    #[derive(Clone)]
    struct SegmentTreeNode {
        start: usize, // start of the subarray covered by the node
        end: usize, 
        max_value: i32, // maximum value in the coverage of the node
    }


    pub struct SegmentTree {
        tree: Vec<SegmentTreeNode>,
    }



    // Constructor for the Segment Tree
    impl SegmentTree {
        pub fn new(size: usize) -> Self {
            // Allocate space for nodes
            let tree_size = 2 * (size.next_power_of_two()) - 1;
            let tree = vec![SegmentTreeNode { start: usize::MAX, end: usize::MAX, max_value:  std::i32::MIN};    tree_size];
            Self { tree }
        }
    }




    // initialization (building the tree from the array)
    impl SegmentTree {



        /// Public function to initialize the segment tree with the given array
        pub fn initialize(&mut self, arr: &Vec<i32>) {
            let n = arr.len();
            self.build_tree(arr, 0, 0, n - 1); // building a tree that covers the whole array
        }


        /// Recursive helper function to build the segment tree
        /// # Arguments:
        /// - arr: the original array
        /// - node: the index of the node (in the array) to be built
        /// - start: the start index of the subarray covered by the node
        /// - end: the end index of the subarray covered by the node
        fn build_tree(&mut self, arr: &Vec<i32>, node: usize, start: usize, end: usize) {
            
            // Set the node's start and end values
            self.tree[node].start = start;
            self.tree[node].end = end;
    

            if start == end { 
                // Leaf node. the value is simply the array element
                self.tree[node].max_value = arr[start];

            } else {
                // Non-leaf node. recursively build its children

                let mid = (start + end) / 2; // midpoint of the subarray

                let left_child = 2 * node + 1; // index of the node in the tree vector
                let right_child = 2 * node + 2; // index of the node in the tree vector
    
                self.build_tree(arr, left_child, start, mid);
                self.build_tree(arr, right_child, mid + 1, end);
    

                // Combine information from children (in this case, the max value)
                let left_child_max = self.tree[left_child].max_value;
                let right_child_max = self.tree[right_child].max_value;

                let max_value = std::cmp::max(left_child_max, right_child_max);
                self.tree[node].max_value = max_value;
            }
        }
    

    }
    




    
    // queries and update
    impl SegmentTree {


        /// Public function to get the maximum value in a range
        pub fn query(&self, query_start: usize, query_end: usize) -> i32 {
            // I substract 1 because, for the queries, the array starts from 1
            // but in the actual implementation, the array starts with 0
            self.query_helper(0, query_start-1, query_end-1)
        }



        /// Recursive function to query the segment tree for the maximum value in a range
        fn query_helper(&self, node: usize, query_start: usize, query_end: usize) -> i32 {
            // println!("query({}, {}, {})", node, query_start, query_end);
            
            // Case 1: No overlap
            if query_end < self.tree[node].start || query_start > self.tree[node].end {
                return std::i32::MIN;
            }
    

            // Case 2: Complete overlap
            if query_start <= self.tree[node].start && query_end >= self.tree[node].end {
                return self.tree[node].max_value;
            }
    

            // Case 3: Partial overlap, recursively go both left and right
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;
    
            let mut left_result = std::i32::MIN;
            let mut right_result = std::i32::MIN;
            let mid = (self.tree[node].start + self.tree[node].end) / 2;

            if query_start <= mid {
                left_result = self.query_helper(left_child, query_start, query_end);
            }
            if query_end > mid {
                right_result = self.query_helper(right_child, query_start, query_end);
            }
    
            // Combine results from children (e.g., max value)
            return std::cmp::max(left_result, right_result);
        }
    

        




        /// Public function to perform an update
        /// # Arguments
        /// - start: the start index of the range to be updated
        /// - end: the end index of the range to be updated
        /// - new_value: the new value to be set IFF it is smaller than the current value
        pub fn update(&mut self, start: usize, end: usize, new_value: i32) {
            // I substract 1 because, for the queries, the array starts from 1
            // but in the actual implementation, the array starts with 0
            self.update_helper(0, start-1, end-1, new_value);
        }


        // Recursive function to update the segment tree
        fn update_helper(&mut self, node: usize, query_start: usize, query_end: usize, new_value: i32) {
            // base case: out of range
            if node > self.tree.len() - 1 {
                return;
            }

            if query_start > self.tree.len() - 1 || query_end > self.tree.len() - 1 {
                return;
            }

            if query_end < self.tree[node].start || query_start > self.tree[node].end {
                return;
            }


            // Base case: I am in a leaf node
            if 
                self.tree[node].start == self.tree[node].end
            {
                // update the max value, if in the range of the query
                if query_start <= self.tree[node].start && query_end >= self.tree[node].end {
                    let new_min = std::cmp::min(self.tree[node].max_value, new_value);
                    self.tree[node].max_value = new_min;
                }
                return;
            }


            // Non-base case: I am in a non-leaf node, update children accordingly
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;
    
            let mid = (self.tree[node].start + self.tree[node].end) / 2;
            
            // now we update the children. We only update the ones that overlap with the query.
            if query_start <= mid {
                self.update_helper(left_child, query_start, query_end, new_value);
            }
            if query_end > mid {
                self.update_helper(right_child, query_start, query_end, new_value);
            }
    

            // Update the current node based on children
            let left_max = self.tree[left_child].max_value;
            let right_max = self.tree[right_child].max_value;
            let new_max = std::cmp::max(left_max, right_max);
            self.tree[node].max_value = new_max;
        }
    

    }




    impl SegmentTree {

        pub fn to_string(&self) -> String {
            let mut result = String::new();
            for i in 0..self.tree.len() {
                result.push_str("------------\n");
                result.push_str(&format!("node_index: {}\n", i));
                result.push_str(&format!("start: {}\n", self.tree[i].start));
                result.push_str(&format!("end: {}\n", self.tree[i].end));
                result.push_str(&format!("max_value: {}\n", self.tree[i].max_value));
                result.push_str("------------\n");
            }
            result
        }
        
    }
    

}



fn main() {
    println!("Hello, segment trees!");
}






// Test function
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;


    // Function to read input from a file and return a tuple (Vec<i32>, Vec<(Query, (usize, usize, usize))>)
    fn read_input(file_path: &str) -> (Vec<i32>, Vec<(usize, usize, usize, usize)>) {
        let path = Path::new(file_path);
        let err_msg = format!("Failed to open file with complete path '{}'", file_path);
        let file = File::open(&path).expect(&err_msg);
        let lines = io::BufReader::new(file).lines();

        let mut iter = lines.map(|line| line.unwrap());

        let _nm: Vec<usize> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let array: Vec<i32> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let queries: Vec<(usize, usize, usize, usize)> = iter
            .map(|line| {
                let values: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                let val_3 = if values.len() > 3 { values[3] } else { 0 };
                (values[0], values[1], values[2], val_3)
            })
            .collect();

        (array, queries)
    }

    // Function to read output from a file and return a Vec<i32>
    fn read_output(file_path: &str) -> Vec<i32> {
        let path = Path::new(file_path);
        let file = File::open(&path).expect("Failed to open file");
        let lines = io::BufReader::new(file).lines();

        lines
            .map(|line| line.unwrap().parse().unwrap())
            .collect()
    }





    #[test]
    fn test_segment_tree() {
        

        // CONFIGS
        // TODO: remove hardcoded configs and pass them as arguments
        let folder_path = "testsets/handson2/"; // Specify the folder where the test files are located
        let debug_print: bool = false; // Set to true to enable debug_print
        let number_of_tests = 10;



        // Attempt to find and run tests for any input and output file pairs
        for i in 0..number_of_tests+1 {
            let input_file = format!("{}input{}.txt", folder_path , i);
            let output_file = format!("{}output{}.txt", folder_path ,  i);

            // Read input and output from files
            let (array, queries) = read_input(&input_file);
            let expected_results = read_output(&output_file);

            let queries_str = format!("Queries: {:?}", queries);

            // Create a new segment tree
            let mut segment_tree = ex_1_segment::SegmentTree::new(array.len());
            segment_tree.initialize(&array);

            // Perform queries and collect results
            let mut results = Vec::new();
            for query in queries {
                if query.0 == 0 {
                    // Update query
                    segment_tree.update(query.1, query.2, query.3 as i32);
                } else {
                    // Max query
                    let result = segment_tree.query(query.1, query.2);
                    results.push(result);
                }
            }


            let mut msg = String::from("=================================\n");
            msg.push_str(&format!("Running test number {}...\n", i));
            msg.push_str(&format!("Input file: {}\n", input_file));
            msg.push_str(&format!("Output file: {}\n", output_file));
            msg.push_str(&format!("Array: {:?}\n", array));
            msg.push_str(&queries_str);
            msg.push_str(&format!("\nExpected results: {:?}\n", expected_results));
            msg.push_str(&format!("Results: {:?}\n", results));
            msg.push_str("Final segment tree:\n");
            msg.push_str(&segment_tree.to_string());
            msg.push_str("=================================\n");
            
            if debug_print { 
                println!("{}", msg);
            }


            // Assert the results match the expected output
            assert_eq!(results, expected_results, "Test files number {} failed!!!" , i);
            println!("Test files number {} passed!", i); 
        }
    }
}

