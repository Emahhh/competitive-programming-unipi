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

        pub fn new(cities: usize, days: usize) -> Self {
            Self { 
                cities, 
                days, 
                itineraries: vec![vec![0; days+1]; cities+1],
                subproblems_table: vec![vec![0; days+1]; cities+1], 
            }
        }

        pub fn set_itinerary(&mut self, itinerary: Vec<usize>) {
            self.itineraries.push(itinerary);
        }

        /// returns the number of attractions you can visit in `city` at the `day` day
        pub fn get_itinerary(&self, city: usize, day: usize) -> usize {
            self.itineraries[city][day]
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
        fn set_solution(&mut self, cities_available: usize, days_available: usize, attractions_num: usize) {
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

                    candidates.push(self.get_solution(city-1, day)); // cell above (same days, but we do not pick the current city at all)

                    for day_pointer in 1..=day {
                        candidates.push(self.get_solution(city-1, day - day_pointer) + self.get_solution(city, day_pointer))
                    }

                    let solution: usize = *candidates.iter().max().unwrap();

                    self.set_solution(city, day, solution);
                }
            }

            return self.get_solution(self.cities, self.days);
        }
    }
}



