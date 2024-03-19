use std::io;
mod utils;
mod local_search;
use local_search::LocalSearch;

fn main() -> io::Result<()> {
    let n = 10;

    let permutation = utils::random_permutation(n);
    println!("Random permutation: {:?}", permutation);

    let (x1, x2) = utils::random_pair(n);
    println!("Random pair: ({}, {})", x1, x2);

    let file_path = "./data/a280.txt";
    let distance_matrix = utils::read_instance(file_path)?;
    let n = distance_matrix[0].len();

    println!("{:?}", distance_matrix[0]);

    let solver = LocalSearch{distance_matrix: distance_matrix, n: n};
    solver.greedy();

    Ok(())
}
