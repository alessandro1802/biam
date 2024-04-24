use std::io;
use std::io::Write;

extern crate glob;
use glob::glob;

mod utils;
mod local_search; mod random; mod simulated_annealing; mod tabu_search;

use local_search::LocalSearch;
use random::Random;
use simulated_annealing::SimulatedAnnealing;
use tabu_search::TabuSearch;

use serde::{Serialize, Deserialize};
use serde_json;


// Struct to save the solution
#[derive(Serialize, Deserialize)]
struct Solution {
    best_distance: f32,
    best_solution: Vec<i64>,
    distances: Vec<f32>,
    runtimes: Vec<u128>,
    steps: Vec<u32>,
    evaluated: Vec<u32>,
}

/**
 * Save the solution to a file in the json format
 *
 * @param instance_name: Name of the instance
 * @param algorithm: Name of the algorithm
 * @param solutions: Vector of solutions
 * @param distances: Vector of distances
 * @param elapsed_time: Vector of elapsed times
 * @param steps: Vector of steps
 * @param evaluated: Vector of evaluated solutions
 */
fn save_solution(instance_name: &str, algorithm: &str, solutions: &Vec<Vec<u32>>, distances: &Vec<f32>, elapsed_time: &Vec<u128>, steps: &Vec<u32>, evaluated: &Vec<u32>) {
    // Schema
    // {
    //    "best_distance": 123.45,
    //    "best_solution": [1, 2, 3, 4, 5],
    //    "distances": [123.45, 123.45, 123.45],
    //    "runtimes": [123, 123, 123]
    //    "steps": [123, 123, 123]
    //    "evaluated": [123, 123, 123]
    // }
    let index_of_min_dist = distances.iter().enumerate().min_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(index, _)| index);
    std::fs::create_dir_all(format!("results/{}", instance_name)).unwrap();
    let file_path = format!("results/{}/{}.json", instance_name, algorithm);
    let mut file = std::fs::File::create(file_path).unwrap();

    // Write the best distance
    let best_distance = distances[index_of_min_dist.unwrap()];
    let best_solution = &solutions[index_of_min_dist.unwrap()];
    let best_solution = best_solution.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
    let best_solution = format!("[{}]", best_solution);
    let best_distance = format!("{:}", best_distance);
    let best_distance = best_distance.parse::<f32>().unwrap();
    let best_solution = serde_json::from_str(&best_solution).unwrap();
    let data = Solution {
        best_distance,
        best_solution,
        distances: distances.clone(),
        runtimes: elapsed_time.clone(),
        steps: steps.clone(),
        evaluated: evaluated.clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}


fn main() -> io::Result<()> {
    // Measurement variables
    let mut time_start;
    let mut avg_time: f64 = 0.0;

//    let algorithms = vec!["greedy", "steepest", "random_search", "random_walk", "heuristic", "simulated_annealing"];
    let algorithms = vec!["tabu_search"];
    let runs = 10;

    for file_path in glob("./data/*.txt").expect("Failed to read glob pattern") {
        let path = file_path.unwrap().display().to_string();
        let instance_name = path.split("/").last().unwrap().split(".").next().unwrap();
        println!("{:?}", instance_name);
        let distance_matrix = utils::read_instance(&path)?;
        let mut solver_LS = LocalSearch::new(distance_matrix.clone());
        let mut solver_R = Random::new(distance_matrix.clone());
        let mut solver_SA = SimulatedAnnealing::new(distance_matrix.clone());
        let mut solver_TS = TabuSearch::new(distance_matrix.clone(), None);
        solver_SA.determine_initial_temperature();

        for algorithm_name in &algorithms {
            let mut elapsed_time = Vec::new();
            let mut distances = Vec::new();
            let mut solutions = Vec::new();
            let mut steps = Vec::new();
            let mut evaluated = Vec::new();
            for _ in 0..runs {
                solver_R.init_random();
                time_start = std::time::Instant::now();
                let (solution, distance, step, eval) = match *algorithm_name {
                    "greedy" => solver_LS.greedy().unwrap(),
                    "steepest" => solver_LS.steepest().unwrap(),
                    "random_search" => solver_R.search(avg_time).unwrap(),
                    "random_walk" => solver_R.walk(avg_time).unwrap(),
                    "heuristic" => solver_LS.heuristic().unwrap(),
                    "simulated_annealing" => solver_SA.run().unwrap(),
                    "tabu_search" => solver_TS.run().unwrap(),
                    _ => panic!("Unknown algorithm"),
                };
                elapsed_time.push(time_start.elapsed().as_millis());
                distances.push(distance);
                solutions.push(solution);
                steps.push(step);
                evaluated.push(eval);
            }
            save_solution(instance_name, algorithm_name, &solutions, &distances, &elapsed_time, &steps, &evaluated);
            avg_time = elapsed_time.iter().sum::<u128>() as f64 / elapsed_time.len() as f64;
            println!("\t{:?}: {:?}", algorithm_name, avg_time);
        }
    }

    Ok(())
}
