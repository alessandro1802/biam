use rand::Rng;
use crate::utils;

/**
* SimulatedAnnealing struct
*
* This struct contains the necessary information to perform simulated annealing
* on a TSP problem. It contains the distance matrix, the number of cities,
* the best solution found and its distance, the current solution and its distance.
*
* @field distance_matrix: The distance matrix of the TSP problem    
* @field n: The number of nodes
*/
pub struct SimulatedAnnealing {
    pub distance_matrix: Vec<Vec<f32>>,
    pub n: usize,
    T: f64,
    alpha: f64,
}

impl SimulatedAnnealing {
    /**
    * Create a new SimulatedAnnealing instance
    *
    * @param distance_matrix: The distance matrix of the TSP problem
    * @return: A new SimulatedAnnealing instance
    */
    pub fn new(distance_matrix: Vec<Vec<f32>>) -> SimulatedAnnealing {
        let n = distance_matrix.len();
        let T = 0.0;

        let alpha = 0.99;
        SimulatedAnnealing {
            distance_matrix,
            n,
            T,
            alpha,
        }
    }

    /**
    * Determine initial temperature.
    */ 
    pub fn determine_initial_temperature(&mut self) {
        let current_tour = utils::random_permutation(self.n);

        let mut rng = rand::thread_rng();
        let sample_n = rng.gen_range(0..self.n / 2);

        let mut n_samples = 0;
        let mut avg_pos_delta: f64 = 0.0;

        // Get a sample of edges
        for i in 0..sample_n {
            let next_i = i + 1;
            for j in i + 2..self.n {
                let next_j = (j + 1) % self.n;
                // Skip directly proceeding edge
                if next_j == i { continue; }
                // Calculated delta fitness
                let delta = utils::get_delta_intra_route(&self.distance_matrix, current_tour[i], current_tour[next_i], current_tour[j], current_tour[next_j]);
                if delta > 0.0 {
                    avg_pos_delta += delta as f64;
                    n_samples += 1;
                }
            }
        }
        avg_pos_delta /= n_samples as f64;
        
        // Set acceptance probability of non-improving solution to 95%;
        self.T = -avg_pos_delta / (0.99_f64).ln();
    }

    
    /**
    * Perform a Simulated Annealing on the TSP problem
    *
    * @return: The best solution found and its distance
    */
    pub fn run(&self) -> Result<(Vec<u32>, f32, u32, u32), &'static str> {
        let mut current_T = self.T;
        let mut current_tour = utils::random_permutation(self.n);
        let mut best_tour = current_tour.clone();

        let mut evaluated = 0;
        let mut steps = 0;
//        let mut no_improvement = 0;

        while current_T > 0.001 {
            let mut accept = false;
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

                    if delta < 0.0 || f64::exp(-delta as f64 / current_T) > rand::random() {
                        best_tour = utils::swap_2_edges(&current_tour, next_i, j, best_tour);
                        accept = true;
                        break;
                    }
                }
                if accept {
                    current_tour = best_tour.clone();
//                    no_improvement = 0;
                    steps += 1;
                    break;
                }
            }
//            if !accept {
//                no_improvement += 1;
//                // Stopping criterion: no improvement
//                if no_improvement >= 10 {
//                    break;
//                }
//            }
            // Decrease the temperature T by multiplying with the cooling rate alpha
            // Geometric decay
//            current_T *= self.alpha;
            // Exponential decay
            current_T = current_T / (1.0 + self.alpha * current_T);
        }
        
        let distance = utils::calculate_tour_distance(&current_tour, &self.distance_matrix).unwrap();
        Ok((current_tour, distance, steps, evaluated))
    }
}
