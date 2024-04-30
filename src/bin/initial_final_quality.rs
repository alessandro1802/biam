use std::io;

use biam::utils;
use biam::local_search::LocalSearch;


fn main() -> io::Result<()> {
    // Measurement variables
    let mut time_start;
    let mut avg_time: f64 = 0.0;

    let instances = vec!["data/a280.txt", "data/berlin52.txt", "data/rat99.txt"];
    let algorithms = vec!["greedy", "steepest"];
    let runs = 500;

    for path in &instances {
        let instance_name = path.split("/").last().unwrap().split(".").next().unwrap();
        println!("{:?}", instance_name);
        let distance_matrix = utils::read_instance(&path)?;
        let mut solver_LS = LocalSearch::new(distance_matrix.clone());

        for algorithm_name in &algorithms {
            let mut init_distances = Vec::new();
            let mut elapsed_time = Vec::new();
            let mut distances = Vec::new();
            let mut solutions = Vec::new();
            let mut steps = Vec::new();
            let mut evaluated = Vec::new();
            for _ in 0..runs {
                let init_sol = utils::random_permutation(distance_matrix.len());
                init_distances.push(utils::calculate_tour_distance(&init_sol, &distance_matrix).unwrap());

                time_start = std::time::Instant::now();
                let (solution, distance, step, eval) = match *algorithm_name {
                    "greedy" => solver_LS.greedy().unwrap(),
                    "steepest" => solver_LS.steepest().unwrap(),
                    _ => panic!("Unknown algorithm"),
                };
                elapsed_time.push(time_start.elapsed().as_millis());
                distances.push(distance);
                solutions.push(solution);
                steps.push(step);
                evaluated.push(eval);
            }
            // Save initial solutions fitenesses as results/init_final/{instance_name}/init_{algorithm_name}
            utils::save_solution(&format!("init_final/{}", instance_name), &format!("init_{}", algorithm_name), &solutions, &init_distances, &elapsed_time, &steps, &evaluated);
            // Save final solutions fitenesses as results/init_final/{instance_name}/final_{algorithm_name}
            utils::save_solution(&format!("init_final/{}", instance_name), &format!("final_{}", algorithm_name), &solutions, &distances, &elapsed_time, &steps, &evaluated);
            avg_time = elapsed_time.iter().sum::<u128>() as f64 / elapsed_time.len() as f64;
            println!("\t{:?}: {:?}", algorithm_name, avg_time);
        }
    }

    Ok(())
}
