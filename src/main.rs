use std::io;
mod utils;

fn main() -> io::Result<()> {
    let n = 10;

    let permutation = utils::random_permutation(n);
    println!("Random permutation: {:?}", permutation);

    let (x1, x2) = utils::random_pair(n);
    println!("Random pair: ({}, {})", x1, x2);


    let file_path = "./data/a280.txt";
    let distance_matrix = utils::read_instance(file_path)?;

    println!("{:?}", distance_matrix[0]);

    Ok(())
}
