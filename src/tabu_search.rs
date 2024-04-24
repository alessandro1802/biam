use crate::utils;

/**
 * TabuSearch structure
 *
 * This struct contains the necessary information to perform tabu search
 * on a TSP problem. It contains the distance matrix, the number of cities,
 * the best solution found and its distance, the current solution and its distance.
 *
 * @field distance_matrix: The distance matrix of the TSP problem
 * @field n: The number of nodes
 */
pub struct TabuSearch {
    pub distance_matrix: Vec<Vec<f32>>,
    pub n: usize,
    tabu_list: Vec<Vec<usize>>,
    tabu_tenure: usize,
    max_iter: u32,
}

impl TabuSearch {
    /**
     * Create a new TabuSearch instance.
     *
     * @param distance_matrix: The distance matrix of the TSP problem.
     * @param iters (optional): The number of iterations to perform without improvement
     * after which the search will stop. Default is 1000.
     * @return: A new TabuSearch instance.
     */
    pub fn new(distance_matrix: Vec<Vec<f32>>, iters: Option<u32>) -> TabuSearch {
        let n = distance_matrix.len();
        // Tabu tenure is set to integer representing size of the problem divided by 4
        let tabu_tenure = n / 4;
        let max_iter: u32 = iters.unwrap_or(100);
        TabuSearch {
            distance_matrix,
            n,
            tabu_list: vec![vec![0; n]; n],
            tabu_tenure,
            max_iter,
        }
    }

    /**
     * Perform a Tabu Search on the TSP problem.
     *
     * @return: The best solution found and its distance.
     */
    pub fn run(&self) -> Result<(Vec<u32>, f32, u32, u32), &'static str> {
        let mut current_tour = utils::random_permutation(self.n);
        let mut best_tour = current_tour.clone();
        let mut best_distance = utils::calculate_tour_distance(&best_tour, &self.distance_matrix).unwrap();
        let mut current_distance = best_distance;
        let mut best_iter: u32 = 0;
        let mut iter: u32 = 0;
        let mut tabu_list = self.tabu_list.clone();
        let mut evaluated: u32 = 0;

        while iter - best_iter < self.max_iter {

            // Generate all possible 2-opt moves, evaluate their delta and sort them
            // in ascending order of delta
            let mut moves: Vec<(usize, usize, f32)> = Vec::new();
            for i in 0..self.n {
                for j in i + 1..self.n {
                    let next_i = (i + 1) % self.n;
                    let next_j = (j + 1) % self.n;
                    if next_j == i { continue; }

                    // Update tabu list
                    if tabu_list[i][j] > 0 {
                        tabu_list[i][j] -= 1;
                    }

                    let delta = utils::get_delta_intra_route(&self.distance_matrix, current_tour[i], current_tour[next_i], current_tour[j], current_tour[next_j]);
                    moves.push((i, j, delta));

                    evaluated += 1;
                }
            }
            moves.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            // Take the first non-tabu move
            for possible_move in moves {
                // If not tabu or move gives better solution than best solution found so far
                if tabu_list[possible_move.0][possible_move.1] == 0 ||
                   current_distance + possible_move.2 < best_distance
                {
                    let next_i = (possible_move.0 + 1) % self.n;
                    current_tour = utils::swap_2_edges(&current_tour, next_i, possible_move.1, best_tour.clone());
                    current_distance += possible_move.2;
                    tabu_list[possible_move.0][possible_move.1] = self.tabu_tenure;
                    break;
                }
            }

            if current_distance < best_distance {
                best_tour = current_tour.clone();
                best_distance = current_distance;
                best_iter = iter;
            }

            iter += 1;
        }

        Ok((best_tour, best_distance, best_iter, evaluated))
    }
}
