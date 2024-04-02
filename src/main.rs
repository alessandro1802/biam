use std::io;
mod utils;
mod local_search;
use local_search::LocalSearch;

fn main() -> io::Result<()> {
    let file_path = "./data/a280.txt";
    let distance_matrix = utils::read_instance(file_path)?;

    let mut solver = LocalSearch::new(distance_matrix);

    solver.init_random();
    let (_solution_r, distance_r) = solver.random(None).unwrap();
    println!("Random: {}", distance_r);

    solver.init_random();
    let (_solution_rw, distance_rw) = solver.random_walk(None).unwrap();
    println!("RandomWalk: {}", distance_rw);

    solver.init_random();
    let (_solution_g, distance_g) = solver.greedy().unwrap();
    println!("Greedy: {}", distance_g);

    solver.init_random();
    let (_solution_s, distance_s) = solver.steepest().unwrap();
    println!("Steepest: {}", distance_s);
    
    solver.init_random();
    let (_solution_h, distance_h) = solver.heuristic().unwrap();
    println!("Heuristic: {}", distance_h);

    Ok(())
}
