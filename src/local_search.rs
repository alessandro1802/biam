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
     * Initialize the LocalSearch instance to random solution
     */
    pub fn init_random(&mut self) {
        self.solution = utils::random_permutation(self.n);
        self.distance = utils::calculate_tour_distance(&self.solution, &self.distance_matrix).unwrap();
        self.current_solution = self.solution.clone();
        self.current_distance = self.distance;
    }

    /**
     * Perform a greedy search for the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn greedy(&mut self) -> Result<(Vec<u32>, f32), &'static str> {
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                self.current_solution.swap(i, j);
                self.current_distance = utils::calculate_tour_distance(&self.current_solution, &self.distance_matrix).unwrap();
                if self.current_distance < self.distance {
                    self.solution = self.current_solution.clone();
                    self.distance = self.current_distance;
                }
                self.current_solution.swap(i, j);
            }
        }
        Ok((self.solution.clone(), self.distance))
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
        let (mut x1, mut x2);
        while self.distance == self.current_distance {
            for _ in 0..samples.unwrap_or(1000) {
                (x1, x2) = utils::random_pair(self.n);
                self.solution.swap(x1, x2);
                self.current_distance = utils::calculate_tour_distance(&self.solution, &self.distance_matrix).unwrap();
                if self.current_distance < self.distance {
                    self.distance = self.current_distance;
                    break;
                }
                self.solution.swap(x1, x2);
            }
        }
        Ok((self.solution.clone(), self.distance))
    }

    /**
     * Perform a steepest search for the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn steepest(&mut self) -> Result<(Vec<u32>, f32), &'static str> {
        Err("Function not implemented")
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
