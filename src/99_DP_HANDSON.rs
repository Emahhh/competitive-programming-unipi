// https://pages.di.unipi.it/rossano/blog/2023/handson32324/

fn main() {
    println!("Hello, dynamic programming!");
}

pub mod holiday_planning {

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

        pub fn new(cities: usize, days: usize, itineraries: Vec<Vec<usize>>) -> Self {
            Self { 
                cities, 
                days, 
                itineraries, 
                subproblems_table: vec![vec![0; days+1]; cities+1], 
            }
        }

        /// returns the number of attractions you can visit in `city` at the `day` day
        pub fn get_itinerary(&self, city: usize, day: usize) -> usize {
            self.itineraries[city][day]
        }

        /// Helper method to get a solution from the table
        fn get_solution(&self, cities_available: usize, days_available: usize) -> usize {
            self.subproblems_table[cities_available][days_available]
        }

        /// Helper ethod to set a solution in the table
        fn set_solution(&mut self, cities_available: usize, days_available: usize, solution: usize) {
            self.subproblems_table[cities_available][days_available] = solution
        }

        /// Method to solve the problem
        /// Returns the number of attractions you can visit, respecting the constraints of the problem
        pub fn solve(&mut self) -> usize {
            
            
        }
    }
}
