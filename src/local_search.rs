use rand::Rng;
use crate::utils;

/**
 * LocalSearch struct
 *
 * This struct contains the necessary information to perform a local search
 * for the TSP problem. It contains the distance matrix, the number of cities,
 * the best solution found and its distance, the current solution and its distance.
 *
 * @field distance_matrix: The distance matrix of the TSP problem
 * @field n: The number of nodes
 * @field solution: The best solution found
 * @field distance: The distance of the best solution found
 * @field current_solution: The current solution
 * @field current_distance: The distance of the current solution
 */
pub struct LocalSearch {
    pub distance_matrix: Vec<Vec<f32>>,
    pub n: usize,
    solution: Vec<u32>,
    distance: f32,
    current_solution: Vec<u32>,
    current_distance: f32,
}

impl LocalSearch {
    /**
     * Create a new LocalSearch instance
     *
     * @param distance_matrix: The distance matrix of the TSP problem
     * @return: A new LocalSearch instance
     */
    pub fn new(distance_matrix: Vec<Vec<f32>>) -> LocalSearch {
        let n = distance_matrix.len();
        let solution = utils::random_permutation(n);
        let distance = utils::calculate_tour_distance(&solution, &distance_matrix).unwrap();
        LocalSearch {
            distance_matrix,
            n,
            solution: solution.clone(),
            distance: distance,
            current_solution: solution.clone(),
            current_distance: distance,
        }
    }

    /**
     * Initialize a random initial solution
     */
    pub fn init_random(&mut self) {
        self.solution = utils::random_permutation(self.n);
        self.distance = utils::calculate_tour_distance(&self.solution, &self.distance_matrix).unwrap();
        self.current_solution = self.solution.clone();
        self.current_distance = self.distance;
    }

    /**
     * Calculate Intra-route delta
     *
     * @param i: The first node of the first edge
     * @param next_i: The second node of the first edge
     * @param j: The first node of the second edge
     * @param next_j: The second node of the second edge
     * @return: The delta fitness
     */
    fn get_delta_intra_route(&self, i: u32, next_i: u32, j: u32, next_j: u32) -> f32 {
        self.distance_matrix[i as usize][j as usize] + self.distance_matrix[next_i as usize][next_j as usize]
        - (self.distance_matrix[i as usize][next_i as usize] + self.distance_matrix[j as usize][next_j as usize])
    }

    /**
     * Swap 2 edges inplace
     *
     * @param current_tour: The current tour
     * @param next_i: The second node of the first edge
     * @param j: The first node of the second edge
     * @param best_tour: The best tour found
     * @return: The best tour found
     */
    fn swap_2_edges(&self, current_tour: &[u32], next_i: usize, j: usize, mut best_tour: Vec<u32>) -> Vec<u32> {
        // Extract the slice [:next_i)
        let first_part = current_tour[..next_i].to_vec();
        // Extract and reverse the slice [next_i, (j + 1))
        let mut middle_part_rev = current_tour[next_i..=j].to_vec();
        middle_part_rev.reverse();
        // Extract the slice [(j + 1):]
        let last_part = current_tour[(j + 1)..].to_vec();
        // Combine the slices
        best_tour.clear();
        best_tour.extend_from_slice(&first_part);
        best_tour.extend_from_slice(&middle_part_rev);
        best_tour.extend_from_slice(&last_part);

        best_tour
    }

    /**
     * Perform a Greedy Local Search on the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn greedy(&mut self) -> Result<(Vec<u32>, f32), &'static str> {
        let mut current_tour = utils::random_permutation(self.n);
        let mut best_tour = current_tour.clone();

        let mut improvement = true;
        while improvement {
            improvement = false;
            // Intra-route neighbourhood: Iterate all distinct 2-edge pairs
            for i in 0..self.n {
                let next_i = (i + 1) % self.n;
                for j in i + 2..self.n {
                    let next_j = (j + 1) % self.n;
                    // Skip directly proceeding edge
                    if next_j == i { continue; }
                    // Calculated delta fitness
                    let delta = self.get_delta_intra_route(current_tour[i], current_tour[next_i], current_tour[j], current_tour[next_j]);

                    if delta < 0.0 {
                        best_tour = self.swap_2_edges(&current_tour, next_i, j, best_tour);
                        improvement = true;
                        break;
                    }
                }
                if improvement {
                    current_tour = best_tour.clone();
                    break;
                }
            }
        }
        let distance = utils::calculate_tour_distance(&current_tour, &self.distance_matrix).unwrap();
        Ok((current_tour, distance))
    }

    /**
     * Perform a Steepest Local Search on the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn steepest(&mut self) -> Result<(Vec<u32>, f32), &'static str> {
        let mut current_tour = utils::random_permutation(self.n);
        let mut best_tour = current_tour.clone();

        let mut best_delta: f32 = 0.0;
        let mut improvement = true;
        while improvement {
            improvement = false;
            // Intra-route neighbourhood: Iterate all distinct 2-edge pairs
            for i in 0..self.n {
                let next_i = (i + 1) % self.n;
                for j in i + 2..self.n {
                    let next_j = (j + 1) % self.n;
                    // Skip directly proceeding edge
                    if next_j == i { continue; }
                    // Calculated delta fitness
                    let delta = self.get_delta_intra_route(current_tour[i], current_tour[next_i], current_tour[j], current_tour[next_j]);

                    if delta < best_delta {
                        best_tour = self.swap_2_edges(&current_tour, next_i, j, best_tour);
                        best_delta = delta;
                        improvement = true;
                    }
                }
            }
            if improvement {
                current_tour = best_tour.clone();
                best_delta = 0.0;
            }
        }

        let distance = utils::calculate_tour_distance(&current_tour, &self.distance_matrix).unwrap();
        Ok((current_tour, distance))
    }

    /**
     * Perform a Random Search on the TSP problem
     *
     * @param time_limit_ms: The time limit in milliseconds
     * @return: The best solution found and its distance
     */
    pub fn random(&mut self, time_limit_ms: f64) -> Result<(Vec<u32>, f32), &'static str> {
        let time_start = std::time::Instant::now();
        while (time_start.elapsed().as_millis() as f64) < time_limit_ms {
            self.current_solution = utils::random_permutation(self.n);
            self.current_distance = utils::calculate_tour_distance(&self.current_solution, &self.distance_matrix).unwrap();
            if self.current_distance < self.distance {
                self.solution = self.current_solution.clone();
                self.distance = self.current_distance;
            }
        }
        Ok((self.solution.clone(), self.distance))
    }

    /**
     * Perform a Random Walk search on the TSP problem
     *
     * @param time_limit_ms: The time limit in milliseconds
     * @return: The best solution found and its distance
     */
    pub fn random_walk(&mut self, time_limit_ms: f64) -> Result<(Vec<u32>, f32), &'static str> {
        let (mut i, mut j, mut delta, mut next_i, mut next_j);
        let time_start = std::time::Instant::now();
        while (time_start.elapsed().as_millis() as f64) < time_limit_ms {
            (i, j) = utils::random_pair(self.n);
            if i > j { std::mem::swap(&mut i, &mut j); }

            next_i = (i + 1) % self.n;
            next_j = (j + 1) % self.n;
            if next_j == i { continue; }

            delta = self.get_delta_intra_route(self.solution[i], self.solution[next_i], self.solution[j], self.solution[next_j]);
            if delta < 0.0 {
                self.solution = self.swap_2_edges(&self.solution, next_i, j, self.solution.clone());
                self.distance = self.distance + delta;
            }
        }
        Ok((self.solution.clone(), self.distance))
    }

    /**
     * Perform a Heuristic search on the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn heuristic(&mut self) -> Result<(Vec<u32>, f32), &'static str> {
        let mut rng = rand::thread_rng();

        let mut visited = vec![false; self.n];
        let mut tour : Vec<u32> = Vec::with_capacity(self.n);
        let mut total_distance = 0.0;

        // Start with a random city
        let mut current_city = rng.gen_range(0..self.n);
        tour.push(current_city as u32);
        visited[current_city] = true;
        // Iterate until all cities are visited
        while tour.len() < self.n {
            let mut min_distance = f32::MAX;
            let mut nearest_city = 0;
            // Find the nearest unvisited city
            for (city, &is_visited) in visited.iter().enumerate() {
                if !is_visited && self.distance_matrix[current_city][city] < min_distance {
                    min_distance = self.distance_matrix[current_city][city];
                    nearest_city = city;
                }
            }
            // Move to the nearest city
            current_city = nearest_city;
            tour.push(current_city as u32);
            visited[current_city] = true;
            total_distance += min_distance;
        }
        // Add distance from the last city back to the starting city
        total_distance += self.distance_matrix[tour[self.n - 1] as usize][tour[0] as usize];

        Ok((tour, total_distance))
    }
}
