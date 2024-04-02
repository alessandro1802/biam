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
     * Initialize the random initial solution
     */
    pub fn init_random(&mut self) {
        self.solution = utils::random_permutation(self.n);
        self.distance = utils::calculate_tour_distance(&self.solution, &self.distance_matrix).unwrap();
        self.current_solution = self.solution.clone();
        self.current_distance = self.distance;
    }

    /**
     * Calculate Intra-route delta
     */
    fn get_delta_intra_route(&self, i: u32, next_i: u32, j: u32, next_j: u32) -> f32 {
        self.distance_matrix[i as usize][j as usize] + self.distance_matrix[next_i as usize][next_j as usize]
        - (self.distance_matrix[i as usize][next_i as usize] + self.distance_matrix[j as usize][next_j as usize])
    }

    /**
     * Swap 2 edges inplace
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
                    // Skip directly preceeding edge
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
                    // Skip directly preceeding edge
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
     * Perform a random solution search for the TSP problem
     *
     * @param samples: Number of samples to generate each iteration (default 1000)
     * @return: The best solution found and its distance
     */
    pub fn random(&mut self, samples: Option<usize>) -> Result<(Vec<u32>, f32), &'static str> {
        while self.distance == self.current_distance {
            for _ in 0..samples.unwrap_or(1000) {
                self.current_solution = utils::random_permutation(self.n);
                self.current_distance = utils::calculate_tour_distance(&self.current_solution, &self.distance_matrix).unwrap();
                if self.current_distance < self.distance {
                    self.solution = self.current_solution.clone();
                    self.distance = self.current_distance;
                    break;
                }
            }
        }
        Ok((self.solution.clone(), self.distance))
    }

    /**
     * Perform a random walk search for the TSP problem
     *
     * @param samples: Number of samples to generate each iteration (default 1000)
     * @return: The best solution found and its distance
     */
    pub fn random_walk(&mut self, samples: Option<usize>) -> Result<(Vec<u32>, f32), &'static str> {
        let (mut x1, mut x2, mut delta);
        let mut continue_search = true;
        while continue_search {
            continue_search = false;
            for _ in 0..samples.unwrap_or(1000) {
                (x1, x2) = utils::random_pair(self.n);
                delta = utils::calculate_delta(&self.solution, &self.distance_matrix, x1, x2).unwrap();
                if delta < 0.0 {
                    self.solution.swap(x1, x2);
                    self.distance = self.distance + delta;
                    continue_search = true;
                    break;
                }
            }
        }
        println!("{:?} {:?}", self.distance, utils::calculate_tour_distance(&self.solution, &self.distance_matrix).unwrap()); // TODO remove print
        Ok((self.solution.clone(), self.distance))
    }



    /**
     * Perform a heuristic search for the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn heuristic(&mut self) -> Result<(Vec<u32>, f32), &'static str> {
        Err("Function not implemented")
    }
}
