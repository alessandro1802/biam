use crate::utils;

pub struct LocalSearch {
    pub distance_matrix: Vec<Vec<f32>>,
    pub n: usize,
}

impl LocalSearch {
    pub fn greedy(&self) {
        let solution = utils::random_permutation(self.n);
        println!("Initial solution: {:?}", solution);
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                println!("Edge: ({}, {})", solution[i], solution[j]);
            }
        }
//        while better_found:
//            # Randomly choose neighbourhood
//            if random.random() > 0.5:
//                    neighbourhood_idx = 1
//            else:
//                neighbourhood_idx = 0
//            if neighbourhood_idx == 0:
//                for i in range(self.targetSolutionSize):
//                    edge1_idx = [i, (i + 1) % self.targetSolutionSize]
//                    edge1 = [current_sol[edge1_idx[0]], current_sol[edge1_idx[1]]]
//                    for j in range(i + 2, self.targetSolutionSize):
//                        if (next_j := (j + 1) % self.targetSolutionSize) == i:
//                            continue
//                        edge2_idx = [j, next_j]
//                        edge2 = [current_sol[edge2_idx[0]], current_sol[edge2_idx[1]]]
//                        # Using nodes themselves
//                        delta = self.getDeltaIntraEdges(edge1, edge2)
//                        if delta < 0:
//                            # Using node indices
//                            # First part, Reversed middle part, Last part
//                            best_route = deepcopy(current_sol)
//                            best_route = best_route[:edge1_idx[1]] + best_route[edge1_idx[1]: (j + 1)][::-1] + best_route[(j + 1):]
//                            break
//                        if best_route:
//                            current_sol = best_route
//                            better_found = True
//                            break
//            # Inter-route
//            else:
//            # Get a list of not selected nodes
//                not_selected = list(set(self.cities) - set(current_sol))
//                for i in range(self.targetSolutionSize):
//                    for node_j in not_selected:
//                        delta = self.getDeltaInter(current_sol[i - 1], current_sol[i], current_sol[(i + 1) % self.targetSolutionSize], node_j)
//                        if delta < 0:
//                            best_route = deepcopy(current_sol)
//                            best_route[i] = node_j
//                            break
//                        if best_route:
//                            current_sol = best_route
//                            better_found = True
//                            break
    }

    /**
     * Perform a random solution search for the TSP problem
     *
     * @param samples: Number of samples to generate each iteration (default 1000)
     * @return: The best solution found
     */
    pub fn random(&self, samples: Option<usize>) -> Result<Vec<u32>, &'static str> {
        let mut solution = utils::random_permutation(self.n);
        let mut distance = utils::calculate_tour_distance(&solution, &self.distance_matrix).unwrap();
        let mut current_solution = solution.clone();
        let mut current_distance = distance;
        let mut continue_search = true;
        while continue_search {
            continue_search = false;
            for _ in 0..samples.unwrap_or(1000) {
                current_solution = utils::random_permutation(self.n);
                current_distance = utils::calculate_tour_distance(&current_solution, &self.distance_matrix).unwrap();
                if current_distance < distance {
                    solution = current_solution.clone();
                    distance = current_distance;
                    continue_search = true;
                    break;
                }
            }
        }
        Ok(solution)
    }

    pub fn steepest(&mut self) {

    }
}
