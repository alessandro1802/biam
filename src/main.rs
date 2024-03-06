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

fn random_pair(n: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    let x1 = rng.gen_range(0..n);
    let x2 = (rng.gen_range(0..(n - 1)) + 1 + x1) % n;

    (x1, x2)
}


fn main() {
    let n = 10;

    let permutation = random_permutation(n);
    println!("Random permutation: {:?}", permutation);

    let (x1, x2) = random_pair(n);
    println!("Random pair: ({}, {})", x1, x2);
}
