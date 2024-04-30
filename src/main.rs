use std::io;

extern crate glob;
use glob::glob;

use biam::utils;

use biam::local_search::LocalSearch;
use biam::random::Random;
use biam::simulated_annealing::SimulatedAnnealing;
use biam::tabu_search::TabuSearch;

fn main() -> io::Result<()> {
    // Measurement variables
    let mut time_start;
    let mut avg_time: f64 = 0.0;

   let algorithms = vec!["greedy", "steepest", "random_search", "random_walk", "heuristic", "simulated_annealing", "tabu_search"];
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
            utils::save_solution(instance_name, algorithm_name, &solutions, &distances, &elapsed_time, &steps, &evaluated);
            avg_time = elapsed_time.iter().sum::<u128>() as f64 / elapsed_time.len() as f64;
            println!("\t{:?}: {:?}", algorithm_name, avg_time);
        }
    }

    Ok(())
}
