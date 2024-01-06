#![allow(unused_imports)]
#![allow(unused_parens)]

// https://pages.di.unipi.it/rossano/blog/2023/handson32324/

const DEBUG: bool = true;
const TESTS_FOLDER: &str = "testsets/handson3-holiday/";

/// # EXERCISE 1
pub mod holiday_planning {
    use super::DEBUG;

    pub struct HolidayProblem {
        /// Number of cities visitable (n)
        cities: usize,

        /// Number of days available (D)
        days: usize,

        /// itineraries[i][j] is the number of attractions you can visit in city i on day j
        itineraries: Vec<Vec<usize>>,

        /// subproblems[i][j] is the number of attractions you can visit with i cities available and j days available
        subproblems_table: Vec<Vec<usize>>,
    }

    impl HolidayProblem {
        pub fn new(cities: usize, days: usize) -> Self {
            Self {
                cities,
                days,
                itineraries: vec![],
                subproblems_table: vec![vec![0; days + 1]; cities + 1],
            }
        }

        pub fn set_itinerary(&mut self, itinerary: Vec<usize>) {
            self.itineraries.push(itinerary);
        }

        /// returns the number of attractions you can visit in `city` at the `day` day
        pub fn get_itinerary(&self, city: usize, day: usize) -> usize {
            self.itineraries[city - 1][day - 1] // -1 because the cities start at 1 in the table
        }

        /// Helper method to get a solution from the table
        /// # Returns
        /// The number of attractions you can visit with `cities_available` cities and `days_available` days
        fn get_solution(&self, cities_available: usize, days_available: usize) -> usize {
            self.subproblems_table[cities_available][days_available]
        }

        /// Helper ethod to set a solution in the table
        /// # Arguments
        /// - cities_available: number of cities available
        /// - days_available: number of days available
        /// - attractions_num: number of attractions you can visit
        fn set_solution(
            &mut self,
            cities_available: usize,
            days_available: usize,
            attractions_num: usize,
        ) {
            self.subproblems_table[cities_available][days_available] = attractions_num
        }

        /// Method to solve the problem
        /// Returns the number of attractions you can visit, respecting the constraints of the problem
        pub fn solve(&mut self) -> usize {
            // filling the first row and column with 0
            for i in 0..=self.days {
                self.set_solution(0, i, 0);
            }

            for i in 0..=self.cities {
                self.set_solution(i, 0, 0);
            }

            for city in 1..=self.cities {
                for day in 1..=self.days {
                    let mut candidates: Vec<usize> = Vec::new();

                    candidates.push(self.get_solution(city - 1, day)); // cell above (same amount of days, but we do not pick the current city at all)

                    // we also have the options where we pick the current city
                    // we have different options: we can pick the city for 1 day, 2 days, 3 days, etc.
                    // the remaining days are going to be spent in other cities (we already have the solutions in the row above)
                    let mut attractions_streak: usize = 0; // accumulates the number of attractions visited in the current city
                    for day_pointer in 1..=day {
                        attractions_streak += self.get_itinerary(city, day_pointer);
                        candidates.push(
                            self.get_solution(city - 1, day - day_pointer) // the best itinerary spent in other cities
                            + attractions_streak,
                        )
                    }

                    let solution: usize = *candidates.iter().max().unwrap();

                    self.set_solution(city, day, solution);
                }
            }

            return self.get_solution(self.cities, self.days);
        }

        pub fn print_subproblems_table(&self) {
            let mut str = String::from("subproblems:\n");

            for i in 0..=self.cities {
                str.push_str(&format!("{:?}\n", self.subproblems_table[i]));
            }
            println!("{}", str);
        }

        pub fn print_itineraries(&self) {
            let mut str = String::from("itineraries:\n");
            for i in 0..self.cities {
                str.push_str(&format!("{:?}\n", self.itineraries[i]));
            }
            println!("{}", str);
        }
    }
}

/// # Tests for EXERCISE 1
#[cfg(test)]
mod tests1 {
    use super::holiday_planning::HolidayProblem;
    use super::*;
    use std::fs;

    fn read_input(filename: &str) -> (usize, usize, Vec<Vec<usize>>) {
        let contents = fs::read_to_string(filename).expect("Failed to read file");
        let mut lines = contents.lines();
        let (cities, days) = {
            let mut split = lines.next().unwrap().split_whitespace();
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        };

        let itineraries: Vec<Vec<usize>> = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();

        if (DEBUG) {
            let mut str = String::from("itineraries from file:\n");
            for i in 0..cities {
                str.push_str(&format!("{:?}\n", itineraries[i]));
            }
            str.push_str(&format!("cities: {}, days: {}\n", cities, days));
            println!("{}", str);
        }

        (cities, days, itineraries)
    }

    fn read_output(filename: &str) -> usize {
        fs::read_to_string(filename)
            .expect("Failed to read file")
            .trim()
            .parse()
            .expect("Failed to parse output")
    }

    /// Runs the first `tests_num` tests in `folder`
    #[test]
    fn test1_range() {
        let folder = TESTS_FOLDER;
        let tests_num = 4;

        for i in 0..=tests_num {
            let input_filename = format!("{}{}", folder, format!("input{}.txt", i));
            let (cities, days, itineraries) = read_input(&input_filename);
            let expected_output_filename = format!("{}{}", folder, format!("output{}.txt", i));
            let expected_output = read_output(&expected_output_filename);

            let mut problem = HolidayProblem::new(cities, days);
            for itinerary in itineraries {
                problem.set_itinerary(itinerary);
            }

            let result = problem.solve();

            if DEBUG {
                // print itineraries
                problem.print_itineraries();

                // print subproblems table
                problem.print_subproblems_table();

                println!("result: {}", result);
            }

            assert_eq!(result, expected_output);
            println!("Test file number {} passed!", i);
        }

        print!("All {} tests passed!\n", tests_num);
    }
}

//--------------------------------------------------------------

/// # EXERCISE 2
///
/// A professor has to prepare a new course.
/// he knows the beauty b_i and the difficulty d_i of each topic i.
/// students appreciate a course only if each lecture is more beautiful than the previous one.
/// the topics must exhibit increasing levels of difficulty.
/// objective is to select the maximum number of topics for his upcoming course.
pub mod course {

    use super::DEBUG;

    #[derive(Debug, Clone, Copy)]
    pub struct Topic {
        /// identifier (used to check if 2 topics are the same topic)
        pub id: usize,

        pub beauty: usize,
        pub difficulty: usize,
    }

    pub struct CourseProblem {
        /// Number of topics (n)
        topics_num: usize,

        /// A list of possible topics for the course.
        topics: Vec<Topic>,
    }

    impl CourseProblem {
        pub fn new(topics_num: usize, topics_vec: Vec<Topic>) -> Self {
            Self {
                topics_num,
                topics: topics_vec,
            }
        }

        /// Solves the course problem.
        ///
        /// # Returns
        ///
        /// A vector of `Topic`(s) that represent a valid selection for the course, given the constraints.
        /// The vector is ordered by increasing difficulty and beauty.
        pub fn solve(&self) -> Vec<Topic> {
            // order two copies of the topics: one by beauty and one by difficulty
            let mut topics_sorted_beauty = self.topics.clone();
            let mut topics_sorted_difficulty = self.topics.clone();

            topics_sorted_beauty.sort_by(|a, b| a.beauty.cmp(&b.beauty));
            topics_sorted_difficulty.sort_by(|a, b| a.difficulty.cmp(&b.difficulty));

            // find the LCS
            let rows = self.topics_num + 1; // +1 because we have a row and column full of 0s
            let cols = self.topics_num + 1;
            let mut mat = vec![vec![0; cols]; rows];

            // filling first col and row with 0s
            for i in 0..rows {
                mat[i][0] = 0;
            }

            for i in 0..cols {
                mat[0][i] = 0;
            }

            // using the algo to fill the rest of the table
            for i in 1..rows {
                for j in 1..cols {
                    if topics_sorted_beauty[i - 1].id == topics_sorted_difficulty[j - 1].id {
                        mat[i][j] = mat[i - 1][j - 1] + 1;
                    } else {
                        mat[i][j] = std::cmp::max(mat[i - 1][j], mat[i][j - 1]);
                    }
                }
            }

            if DEBUG {
                let mut output = String::new();
                for row in &mat {
                    output.push_str(&format!("{:?}\n", row));
                }
                println!("{}", output);
            }

            // reconstruct the solution (the selected topics)
            let mut i = rows - 1;
            let mut j = cols - 1;
            let mut topics = Vec::new();
            while i > 0 && j > 0 {
                if topics_sorted_beauty[i - 1].id == topics_sorted_difficulty[j - 1].id {
                    topics.push(topics_sorted_beauty[i - 1]);
                    i -= 1;
                    j -= 1;
                } else if mat[i - 1][j] > mat[i][j - 1] {
                    i -= 1;
                } else {
                    j -= 1;
                }
            }

            topics.reverse(); // nonstrictly strictly increasing

            // check that `topics` is STRICTLY increasing, by removing consecutive topics that have same beauty or difficulty
            let mut k = 0;
            while k < topics.len() - 1 {
                if topics[k].beauty == topics[k + 1].beauty
                    || topics[k].difficulty == topics[k + 1].difficulty
                {
                    topics.remove(k);
                } else {
                    k += 1;
                }
            }

            // topics now contains an ordered selection of topics that satisfy the requirements (STRICTLY increasing)

            topics
        }
    }
}

/// # Tests for EXERCISE 2
///
/// # Input
/// The first line contains n. Each of the next n lines contains the beuaty b and the difficulty d, one for each topic.
///
/// Example:
/// ```
/// 5      // n
/// 0 3    // beauty 0 and difficulty 3. Write me an email if you know what this topic is.
/// 99 1   // Fenwick tree?
/// 11 20
/// 1 2
/// 10 5
/// ````
///
/// # Output
/// The output files contains the largest number of selected topics.
#[cfg(test)]
mod tests2 {
    use super::course::*;
    use super::*;
    use std::fs;

    fn read_input(filename: &str) -> (usize, Vec<Topic>) {
        let contents = fs::read_to_string(filename).expect("Failed to read file");
        let mut lines = contents.lines();
        let topics_num = lines.next().unwrap().parse().unwrap();

        let mut id_counter: usize = 0;

        let topics: Vec<Topic> = lines
            .map(|line| {
                let mut split = line.split_whitespace();
                let my_topic = Topic {
                    id: id_counter,
                    beauty: split.next().unwrap().parse().unwrap(),
                    difficulty: split.next().unwrap().parse().unwrap(),
                };
                id_counter += 1;
                return my_topic;
            })
            .collect();

        (topics_num, topics)
    }

    fn read_output(filename: &str) -> usize {
        fs::read_to_string(filename)
            .expect("Failed to read file")
            .trim()
            .parse()
            .expect("Failed to parse output")
    }

    /// Runs the first `tests_num` tests in `folder`
    #[test]
    fn tests2_range() {
        let folder = "testsets/handson3-course";
        let tests_num = 7;

        for i in 0..=tests_num {
            let input_filename = format!("{}/input{}.txt", folder, i);
            let (topics_num, topics) = read_input(&input_filename);
            let expected_output_filename = format!("{}/output{}.txt", folder, i);
            let expected_output = read_output(&expected_output_filename);

            let problem = CourseProblem::new(topics_num, topics);
            let topics = problem.solve(); // topics now contains a selection of topics that satisfy the requirements
            let topics_num = topics.len();

            if DEBUG {
                println!("{:?} topics found: {:?}\n\n", topics_num, topics);
            }

            assert_eq!(topics_num, expected_output);
            println!("Test file number {} passed!", i);
        }

        print!("All {} tests passed!\n", tests_num);
    }
}

fn main() {
    println!("Hello, dynamic programming!");
}
