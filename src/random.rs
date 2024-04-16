use crate::utils;

/**
    * Random struct
    *
    * This struct contains the necessary information to perform random algorithms
    * on a TSP problem. It contains the distance matrix, the number of cities,
    * the best solution found and its distance, the current solution and its distance.
    *
    * @field distance_matrix: The distance matrix of the TSP problem
    * @field n: The number of nodes
    * @field solution: The best solution found
    * @field distance: The distance of the best solution found
    * @field current_solution: The current solution
    * @field current_distance: The distance of the current solution
    */
pub struct Random {
    pub distance_matrix: Vec<Vec<f32>>,
    pub n: usize,
    solution: Vec<u32>,
    distance: f32,
    current_solution: Vec<u32>,
    current_distance: f32,
}

impl Random {
    /**
     * Create a new Random instance
     *
     * @param distance_matrix: The distance matrix of the TSP problem
     * @return: A new Random instance
     */
    pub fn new(distance_matrix: Vec<Vec<f32>>) -> Random {
        let n = distance_matrix.len();
        let solution = utils::random_permutation(n);
        let distance = utils::calculate_tour_distance(&solution, &distance_matrix).unwrap();
        Random {
            distance_matrix,
            n,
            solution: solution.clone(),
            distance: distance,
            current_solution: solution.clone(),
            current_distance: distance,
        }
    }

    /**
     * Initialize a random solution
     */
    pub fn init_random(&mut self) {
        self.solution = utils::random_permutation(self.n);
        self.distance = utils::calculate_tour_distance(&self.solution, &self.distance_matrix).unwrap();
        self.current_solution = self.solution.clone();
        self.current_distance = self.distance;
    }
    
    
    /**
     * Perform a Random Search on the TSP problem
     *
     * @param time_limit_ms: The time limit in milliseconds
     * @return: The best solution found and its distance
     */
    pub fn search(&mut self, time_limit_ms: f64) -> Result<(Vec<u32>, f32, u32, u32), &'static str> {
        let mut evaluated = 0;

        let time_start = std::time::Instant::now();
        while (time_start.elapsed().as_millis() as f64) < time_limit_ms {
            self.current_solution = utils::random_permutation(self.n);
            self.current_distance = utils::calculate_tour_distance(&self.current_solution, &self.distance_matrix).unwrap();
            evaluated += 1;
            if self.current_distance < self.distance {
                self.solution = self.current_solution.clone();
                self.distance = self.current_distance;
            }
        }
        Ok((self.solution.clone(), self.distance, 0, evaluated))
    }
    
    /**
     * Perform a Random Walk search on the TSP problem
     *
     * @param time_limit_ms: The time limit in milliseconds
     * @return: The best solution found and its distance
     */
    pub fn walk(&mut self, time_limit_ms: f64) -> Result<(Vec<u32>, f32, u32, u32), &'static str> {
        let mut evaluated = 0;

        let (mut i, mut j, mut delta, mut next_i, mut next_j);
        let time_start = std::time::Instant::now();
        while (time_start.elapsed().as_millis() as f64) < time_limit_ms {
            (i, j) = utils::random_pair(self.n);
            if i > j { std::mem::swap(&mut i, &mut j); }

            next_i = (i + 1) % self.n;
            next_j = (j + 1) % self.n;
            if next_j == i { continue; }

            delta = utils::get_delta_intra_route(&self.distance_matrix, self.solution[i], self.solution[next_i], self.solution[j], self.solution[next_j]);
            evaluated += 1;

            if delta < 0.0 {
                self.solution = utils::swap_2_edges(&self.solution, next_i, j, self.solution.clone());
                self.distance = self.distance + delta;
            }
        }
        Ok((self.solution.clone(), self.distance, 0, evaluated))
    }
}
