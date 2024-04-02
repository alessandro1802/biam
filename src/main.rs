use std::io;
use std::io::Write;
mod utils;
mod local_search;
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
    let file_path = "./data/a280.txt";
    let instance_name = file_path.split("/").last().unwrap().split(".").next().unwrap();
    let distance_matrix = utils::read_instance(file_path)?;
    let mut solver = LocalSearch::new(distance_matrix);

    // Measurement variables
    let mut time_start;
    let mut elapsed_time = Vec::new();
    let mut distances = Vec::new();
    let mut solutions = Vec::new();

    // Run the steepest algorithm 10 times
    for _ in 0..10 {
        solver.init_random();
        time_start = std::time::Instant::now();
        let (solution_s, distance_s) = solver.steepest().unwrap();
        elapsed_time.push(time_start.elapsed().as_millis());
        distances.push(distance_s);
        solutions.push(solution_s);
    }
    save_solution(instance_name, "steepest", &solutions, &distances, &elapsed_time);
    let avg_time_s = elapsed_time.iter().sum::<u128>() as f64 / elapsed_time.len() as f64;
    elapsed_time.clear();
    distances.clear();
    solutions.clear();

    // Run the random algorithm 10 times
    for _ in 0..10 {
        solver.init_random();
        time_start = std::time::Instant::now();
        let (solution_r, distance_r) = solver.random(None).unwrap();
        elapsed_time.push(time_start.elapsed().as_millis());
        distances.push(distance_r);
        solutions.push(solution_r);
    }
    save_solution(instance_name, "random", &solutions, &distances, &elapsed_time);
    elapsed_time.clear();
    distances.clear();
    solutions.clear();

    // Run the random walk algorithm 10 times
    for _ in 0..10 {
        solver.init_random();
        time_start = std::time::Instant::now();
        let (solution_rw, distance_rw) = solver.random_walk(None).unwrap();
        elapsed_time.push(time_start.elapsed().as_millis());
        distances.push(distance_rw);
        solutions.push(solution_rw);
    }
    save_solution(instance_name, "random_walk", &solutions, &distances, &elapsed_time);
    elapsed_time.clear();
    distances.clear();
    solutions.clear();

    // Run the greedy algorithm 10 times
    for _ in 0..10 {
        solver.init_random();
        time_start = std::time::Instant::now();
        let (solution_g, distance_g) = solver.greedy().unwrap();
        elapsed_time.push(time_start.elapsed().as_millis());
        distances.push(distance_g);
        solutions.push(solution_g);
    }
    save_solution(instance_name, "greedy", &solutions, &distances, &elapsed_time);
    elapsed_time.clear();
    distances.clear();
    solutions.clear();

    // Run the heuristic algorithm 10 times
    for _ in 0..10 {
        solver.init_random();
        time_start = std::time::Instant::now();
        let (solution_h, distance_h) = solver.heuristic().unwrap();
        elapsed_time.push(time_start.elapsed().as_millis());
        distances.push(distance_h);
        solutions.push(solution_h);
    }
    save_solution(instance_name, "heuristic", &solutions, &distances, &elapsed_time);

    Ok(())
}
