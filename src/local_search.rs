use rand::Rng;
use crate::utils;

/**
 * LocalSearch struct
 *
 * This struct contains the necessary information to perform local search
 * on a TSP problem. It contains the distance matrix, the number of cities,
 * the best solution found and its distance, the current solution and its distance.
 *
 * @field distance_matrix: The distance matrix of the TSP problem
 * @field n: The number of nodes
 */
pub struct LocalSearch {
    pub distance_matrix: Vec<Vec<f32>>,
    pub n: usize,
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
        LocalSearch {
            distance_matrix,
            n,
        }
    }


    /**
     * Perform a Greedy Local Search on the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn greedy(&mut self) -> Result<(Vec<u32>, f32, u32, u32), &'static str> {
        let mut current_tour = utils::random_permutation(self.n);
        let mut best_tour = current_tour.clone();

        let mut evaluated = 0;
        let mut steps = 0;

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
                    let delta = utils::get_delta_intra_route(&self.distance_matrix,current_tour[i], current_tour[next_i], current_tour[j], current_tour[next_j]);
                    evaluated += 1;

                    if delta < 0.0 {
                        best_tour = utils::swap_2_edges(&current_tour, next_i, j, best_tour);
                        improvement = true;
                        break;
                    }
                }
                if improvement {
                    current_tour = best_tour.clone();
                    steps += 1;
                    break;
                }
            }
        }
        let distance = utils::calculate_tour_distance(&current_tour, &self.distance_matrix).unwrap();
        Ok((current_tour, distance, steps, evaluated))
    }

    /**
     * Perform a Steepest Local Search on the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn steepest(&mut self) -> Result<(Vec<u32>, f32, u32, u32), &'static str> {
        let mut current_tour = utils::random_permutation(self.n);
        let mut best_tour = current_tour.clone();

        let mut evaluated = 0;
        let mut steps = 0;

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
                    let delta = utils::get_delta_intra_route(&self.distance_matrix, current_tour[i], current_tour[next_i], current_tour[j], current_tour[next_j]);
                    evaluated += 1;

                    if delta < best_delta {
                        best_tour = utils::swap_2_edges(&current_tour, next_i, j, best_tour);
                        best_delta = delta;
                        improvement = true;
                    }
                }
            }
            if improvement {
                current_tour = best_tour.clone();
                steps += 1;
                best_delta = 0.0;
            }
        }

        let distance = utils::calculate_tour_distance(&current_tour, &self.distance_matrix).unwrap();
        Ok((current_tour, distance, steps, evaluated))
    }


    /**
     * Perform a Heuristic search on the TSP problem
     *
     * @return: The best solution found and its distance
     */
    pub fn heuristic(&mut self) -> Result<(Vec<u32>, f32, u32, u32), &'static str> {
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

        Ok((tour, total_distance, 0, 0))
    }
}
