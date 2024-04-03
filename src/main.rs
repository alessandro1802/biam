use std::io;
use std::io::Write;
extern crate glob;
use glob::glob;
mod utils; mod local_search;
use local_search::LocalSearch;

/**
 * Save the solution to a file
 *
 * @param instance_name: Name of the instance
 * @param algorithm: Name of the algorithm
 * @param solutions: Vector of solutions
 * @param distances: Vector of distances
 * @param elapsed_time: Vector of elapsed times
 */
fn save_solution(instance_name: &str, algorithm: &str, solutions: &Vec<Vec<u32>>, distances: &Vec<f32>, elapsed_time: &Vec<u128>) {
    let index_of_min_dist = distances.iter().enumerate().min_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(index, _)| index);
    std::fs::create_dir_all(format!("results/{}", instance_name)).unwrap();
    let file_path = format!("results/{}/{}.txt", instance_name, algorithm);
    let mut file = std::fs::File::create(file_path).unwrap();

    // Write the best distance
    let mut result = writeln!(file, "Best distance:\n{}", distances[index_of_min_dist.unwrap()]);
    result.expect("Error writing to file");

    // Write the best solution
    result = writeln!(file, "Best solution:");
    result.expect("Error writing to file");
    for i in 0..solutions[index_of_min_dist.unwrap()].len()-1 {
        result = write!(file, "{} ", solutions[index_of_min_dist.unwrap()][i]);
        result.expect("Error writing to file");
    }
    result = writeln!(file, "{}", solutions[index_of_min_dist.unwrap()][solutions[index_of_min_dist.unwrap()].len()-1]);
    result.expect("Error writing to file");

    // Write the distances and elapsed times
    result = writeln!(file, "Distances:");
    result.expect("Error writing to file");
    for i in 0..distances.len()-1 {
        result = write!(file, "{} ", distances[i]);
        result.expect("Error writing to file");
    }
    result = writeln!(file, "{}", distances[distances.len()-1]);
    result.expect("Error writing to file");

    // Write the runtimes
    result = writeln!(file, "Runtimes:");
    result.expect("Error writing to file");
    for i in 0..elapsed_time.len()-1 {
        result = write!(file, "{} ", elapsed_time[i]);
        result.expect("Error writing to file");
    }
    result = writeln!(file, "{}", elapsed_time[elapsed_time.len()-1]);
    result.expect("Error writing to file");
}


fn main() -> io::Result<()> {
    // Measurement variables
    let mut time_start;
    let mut avg_time: f64 = 0.0;
    let mut elapsed_time = Vec::new();
    let mut distances = Vec::new();
    let mut solutions = Vec::new();

    let algorithms = vec!["greedy", "steepest", "random", "random_walk", "heuristic"];
    let runs = 10;

    for file_path in glob("./data/*.txt").expect("Failed to read glob pattern") {
        //        println!("{}", file_path.unwrap().display());
        let path = file_path.unwrap().display().to_string();
        let instance_name = path.split("/").last().unwrap().split(".").next().unwrap();
        println!("{:?}", instance_name);
        let distance_matrix = utils::read_instance(&path)?;
        let mut solver = LocalSearch::new(distance_matrix);

        for algorithm_name in &algorithms {
            for _ in 0..runs {
                solver.init_random();
                time_start = std::time::Instant::now();
                let (solution, distance, steps, evaluated) = match *algorithm_name {
                    "greedy" => solver.greedy().unwrap(),
                    "steepest" => solver.steepest().unwrap(),
                    "random" => solver.random(avg_time).unwrap(),
                    "random_walk" => solver.random_walk(avg_time).unwrap(),
                    "heuristic" => solver.heuristic().unwrap(),
                    _ => panic!("Unknown algorithm"),
                };
                elapsed_time.push(time_start.elapsed().as_millis());
                distances.push(distance);
                solutions.push(solution);
            }
            save_solution(instance_name, algorithm_name, &solutions, &distances, &elapsed_time);
            avg_time = elapsed_time.iter().sum::<u128>() as f64 / elapsed_time.len() as f64;
            println!("\t{:?}: {:?}", algorithm_name, avg_time);
            elapsed_time.clear();
            distances.clear();
            solutions.clear();
        }
    }

    Ok(())
}
