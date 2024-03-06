use rand::prelude::*;

fn random_permutation(n: usize) -> Vec<usize> {
    let mut permutation: Vec<usize> = (0..n).collect();
    let mut rng = rand::thread_rng();

    for i in (1..n).rev() {
        let j = rng.gen_range(0..=i);
        permutation.swap(i, j);
    }

    permutation
}

fn main() {
    let n = 10;
    let permutation = random_permutation(n);
    println!("Random permutation: {:?}", permutation);
}
