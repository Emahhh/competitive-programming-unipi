#![allow(unused_imports)]
#![allow(unused_parens)]



fn main() {
    println!("Hello, dynamic programming!");
}

/* trunk-ignore(clippy/dead_code) */
const DEBUG: bool = false;
const TESTS_FOLDER: &str = "testsets/handson3-holiday/";

// https://pages.di.unipi.it/rossano/blog/2023/handson32324/

pub mod holiday_planning {
    use super::DEBUG;

    pub struct Problem {
        /// Number of cities visitable (n)
        cities: usize,

        /// Number of days available (D)
        days: usize,

        /// itineraries[i][j] is the number of attractions you can visit in city i on day j
        itineraries: Vec<Vec<usize>>,

        /// subproblems[i][j] is the number of attractions you can visit with i cities available and j days available
        subproblems_table: Vec<Vec<usize>>,
    }

    impl Problem {
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
            self.itineraries[city-1][day-1] // -1 because the cities start at 1 in the table
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
                    let mut attractions_streak : usize = 0; // accumulates the number of attractions visited in the current city
                    for day_pointer in 1..=day {
                        attractions_streak += self.get_itinerary(city, day_pointer);
                        candidates.push(
                            self.get_solution(city - 1, day - day_pointer) // the best itinerary spent in other cities
                            + attractions_streak
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::holiday_planning::Problem;
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

        if (DEBUG){
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


    #[test]
    fn test_range() {
        let folder = TESTS_FOLDER;
        let tests_num = 4;

        for i in 0..=tests_num {
            let input_filename = format!("{}{}", folder, format!("input{}.txt", i));
            let (cities, days, itineraries) = read_input(&input_filename);
            let expected_output_filename = format!("{}{}", folder, format!("output{}.txt", i));
            let expected_output = read_output(&expected_output_filename);

            let mut problem = Problem::new(cities, days);
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
