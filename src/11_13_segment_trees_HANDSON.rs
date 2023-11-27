#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::items_after_test_module)]

// https://pages.di.unipi.it/rossano/blog/2023/handson22324/


fn main() {
    println!("Hello, segment trees!");
}



/// EXERCISE 1
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






// TESTS FOR EXERCISE 1
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
        let folder_path = "testsets/handson2-minmax/"; // Specify the folder where the test files are located
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







/// # Author
/// https://pages.di.unipi.it/rossano/blog/2023/fenwick/
pub mod fenwick {
    #[derive(Debug)]
    pub struct FenwickTree {
        tree: Vec<i64>,
    }
    
    impl FenwickTree {
        pub fn with_len(n: usize) -> Self {
            Self {
                tree: vec![0; n + 1],
            }
        }
    
        pub fn len(&self) -> usize {
            self.tree.len() - 1
        }
    
        /// Indexing is 0-based, even if internally we use 1-based indexing
        pub fn add(&mut self, i: usize, delta: i64) {
            let mut i = i + 1; 
            assert!(i < self.tree.len(), "Index out of bounds. Tried to add {} to index {}, but tree length is {}", delta, i, self.tree.len());
    
            while i < self.tree.len() {
                self.tree[i] += delta;
                i = Self::next_sibling(i);
            }
        }
    
        /// Indexing is 0-based, even if internally we use 1-based indexing
        pub fn sum(&self, i: usize) -> i64 {
            let mut i = i + 1;  
    
            assert!(i < self.tree.len());
            let mut sum = 0;
            while i != 0 {
                sum += self.tree[i];
                i = Self::parent(i);
            }
    
            sum
        }
    
        pub fn range_sum(&self, l: usize, r: usize) -> i64 {
            self.sum(r) - if l == 0 { 0 } else { self.sum(l - 1) }
        }
    
        fn isolate_trailing_one(i: usize) -> usize {
            if i == 0 {
                0
            } else {
                1 << i.trailing_zeros()
            }
        }
    
        fn parent(i: usize) -> usize {
            i - Self::isolate_trailing_one(i)
        }
    
        fn next_sibling(i: usize) -> usize {
            i + Self::isolate_trailing_one(i)
        }




    }
}




/// EXERCISE 2 (is_there)
/// I used the fenwick tree implementation seen during the course.
/// I used a similar approach as seen in another exercise to represent the segments.
/// The is_there query is not optimal, amd should be optimized.
pub mod ex_2_is_there {
    use crate::fenwick::FenwickTree;

    use super::fenwick;

    pub struct IsThereExercise {
        tree: fenwick::FenwickTree
    }

    impl IsThereExercise {

        /// Create a new IsThereExercise
        /// # Arguments 
        /// * `n` - The maximum length of a segment, that also equals the number of segments
        /// * `segments` - A vector containing couples that represent segments. Every couple contains indexes that represent the start and the end of the segment.
        pub fn new(n: usize,segments: &Vec<(usize, usize)>) -> Self {

            let mut tree = fenwick::FenwickTree::with_len(n*3);

            for (start, end) in segments.iter(){
                let mut end_index = *end+1;
                tree.add(end_index, -1);
                tree.add(*start, 1);
            }

            Self { tree }
        }


        // TODO: optimize, maybe by using binary search
        pub fn is_there(&self, start_range: usize, end_range: usize, num_of_segments: usize) -> usize {
            for i in start_range..=end_range {
                if self.tree.sum(i) == num_of_segments as i64 {
                    return 1;
                }
            }

            return 0;
        }
        
    }


    impl FenwickTree {

        pub fn to_string(&self) -> String {
            let mut s = String::new();
            
            for i in 0..self.len() {
                s.push_str( &format!("{:?} ", self.sum(i)) );
            }

            return s;
        }

    }

    impl IsThereExercise {
        pub fn tree_to_string(&self) -> String {
            self.tree.to_string()
        }
    }






}







// TESTS FOR IsThereExercise
#[cfg(test)]
mod ex_2_tests {
    use super::ex_2_is_there::IsThereExercise;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    // Function to read input from a file and return a tuple (usize, Vec<(usize, usize)>, Vec<(usize, usize, usize)>)
    fn read_input(file_path: &str) -> (usize, Vec<(usize, usize)>, Vec<(usize, usize, usize)>) {
        let path = Path::new(file_path);
        let err_msg = format!("Failed to open file with complete path '{}'", file_path);
        let file = File::open(&path).expect(&err_msg);
        let lines = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect::<Vec<_>>();

        let iter = lines.iter().cloned();

        let n_m: Vec<usize> = iter
            .clone()
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let n = n_m[0];
        let m = n_m[1];

        let segments: Vec<(usize, usize)> = iter
            .clone()
            .skip(1)
            .take(n)
            .map(|line| {
                let values: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                (values[0], values[1])
            })
            .collect();

        let queries: Vec<(usize, usize, usize)> = iter
            .skip(1 + n)
            .take(m)
            .map(|line| {
                let values: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                (values[0], values[1], values[2])
            })
            .collect();

        (m, segments, queries)
    }

    // Function to read output from a file and return a Vec<usize>
    fn read_output(file_path: &str) -> Vec<usize> {
        let path = Path::new(file_path);
        let file = File::open(&path).expect("Failed to open file");
        let lines = io::BufReader::new(file).lines();

        lines
            .map(|line| line.unwrap().parse().unwrap())
            .collect()
    }






    #[test]
    fn test_is_there() {
        // CONFIGS
        let folder_path = "testsets/handson2-isthere/";
        let number_of_tests = 7;

        for i in 0..number_of_tests + 1 {
            let input_file = format!("{}input{}.txt", folder_path, i);
            let output_file = format!("{}output{}.txt", folder_path, i);

            // Read input and output from files
            let (m, segments, queries) = read_input(&input_file);
            let expected_results = read_output(&output_file);

            // Create a new IsThereExercise
            let exercise = IsThereExercise::new(m, &segments);

            // Perform queries and collect results
            let mut results = Vec::new();
            for query in queries {
                let result = exercise.is_there(query.0, query.1, query.2);
                results.push(result);
            }

            // println!("Tree {}: {:?}", i, exercise.tree_to_string());

            // Assert the results match the expected output
            assert_eq!(results, expected_results, "Test files number {} failed!!!" , i);
            println!("Test files number {} passed!", i);
        }
    }
}
