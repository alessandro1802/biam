use std::fs::File;
use std::io::{self, BufRead};

use rand::prelude::*;


struct Coordinate {
    x: u32,
    y: u32,
}

fn euclidean_distance(coord1: &Coordinate, coord2: &Coordinate) -> f32 {
    ((coord2.x as f32 - coord1.x as f32).powi(2) + (coord2.y as f32 - coord1.y as f32).powi(2)).sqrt()
}

fn calculate_distance_matrix(coordinates: &[Coordinate]) -> Vec<Vec<f32>> {
    let mut distance_matrix = vec![vec![0.0; coordinates.len()]; coordinates.len()];
    for i in 0..coordinates.len() {
        for j in 0..coordinates.len() {
            let dist = euclidean_distance(&coordinates[i], &coordinates[j]);
            distance_matrix[i][j] = dist;
            distance_matrix[j][i] = dist;
        }
    }
    distance_matrix
}

pub fn read_instance(file_path: &str) -> io::Result<Vec<Vec<f32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut coordinates = Vec::new();
    let mut reading_coordinates = false;
    for line in reader.lines() {
        let line = line?;
        if line == "NODE_COORD_SECTION" {
            reading_coordinates = true;
            continue;
        } else if line == "EOF" {
            break;
        }

        if reading_coordinates {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let (Ok(x), Ok(y)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                    coordinates.push(Coordinate { x, y });
                }
            }
        }
    }
    let distance_matrix = calculate_distance_matrix(&coordinates);
    Ok(distance_matrix)
}


pub fn random_permutation(n: usize) -> Vec<usize> {
    let mut permutation: Vec<usize> = (0..n).collect();
    let mut rng = rand::thread_rng();

    for i in (1..n).rev() {
        let j = rng.gen_range(0..=i);
        permutation.swap(i, j);
    }

    permutation
}

pub fn random_pair(n: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    let x1 = rng.gen_range(0..n);
    let x2 = (rng.gen_range(0..(n - 1)) + 1 + x1) % n;

    (x1, x2)
}
