use std::fs::File;
use std::io::{self, BufRead};

use rand::prelude::*;


/**
 * A struct to represent a coordinate in a 2D plane.
 *
 * @field x: The x-coordinate of the point.
 * @field y: The y-coordinate of the point.
 */
struct Coordinate {
    x: f32,
    y: f32,
}

/**
 * Calculate the Euclidean distance between two coordinates.
 *
 * @param coord1: The first coordinate.
 * @param coord2: The second coordinate.
 * @return The Euclidean distance between the two coordinates.
 */
fn euclidean_distance(coord1: &Coordinate, coord2: &Coordinate) -> f32 {
    ((coord2.x as f32 - coord1.x as f32).powi(2) + (coord2.y as f32 - coord1.y as f32).powi(2)).sqrt()
}

/**
 * Calculate the distance matrix between a set of coordinates.
 *
 * @param coordinates: The set of coordinates.
 * @return The distance matrix between the coordinates.
 */
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

/**
 * Calculates the total distance of a tour.
 *
 * @param tour: The tour.
 * @param distance_matrix: The distance matrix between the coordinates.
 * @return The total distance of the tour.
 */
pub fn calculate_tour_distance(tour: &[u32], distance_matrix: &[Vec<f32>]) -> io::Result<f32> {
    let mut distance = 0.0;
    for i in 0..tour.len() {
        distance += distance_matrix[tour[i] as usize][tour[(i + 1) % tour.len()] as usize];
    }
    Ok(distance)
}

/**
 * Calculate delta of the total distances of a tour after swapping two edges.
 * The delta is the difference between the total distances of the tours before and after the swap.
 *
 * @param tour: The tour.
 * @param distance_matrix: The distance matrix between the coordinates.
 * @param i: The first index.
 * @param j: The second index.
 * @return The delta of the total distances of the tours after swapping two edges.
 */
pub fn calculate_delta(tour: &[u32], distance_matrix: &[Vec<f32>], i: usize, j: usize) -> io::Result<f32> {
    let n = tour.len();
    let el_before_i = tour[(i + n - 1) % n] as usize;
    let el_after_i = tour[(i + 1) % n] as usize;
    let el_before_j = tour[(j + n - 1) % n] as usize;
    let el_after_j = tour[(j + 1) % n] as usize;

    let mut delta = 0.0;

    delta += distance_matrix[el_before_i][tour[i] as usize];
    delta += distance_matrix[tour[i] as usize][el_after_i];
    delta += distance_matrix[el_before_j][tour[j] as usize];
    delta += distance_matrix[tour[j] as usize][el_after_j];

    delta -= distance_matrix[el_before_i][tour[j] as usize];
    delta -= distance_matrix[tour[j] as usize][el_after_i];
    delta -= distance_matrix[el_before_j][tour[i] as usize];
    delta -= distance_matrix[tour[i] as usize][el_after_j];

    Ok(delta)
}

/**
 * Read an instance from a file.
 *
 * @param file_path: The path to the file.
 * @return The distance matrix between the coordinates.
 */
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
                if let (Ok(x), Ok(y)) = (parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                    coordinates.push(Coordinate { x, y });
                }
            }
        }
    }
    let distance_matrix = calculate_distance_matrix(&coordinates);
    Ok(distance_matrix)
}

/**
 * Calculate the total distance of a tour.
 *
 * @param tour: The tour.
 * @param distance_matrix: The distance matrix between the coordinates.
 * @return The total distance of the tour.
 */
pub fn random_permutation(n: usize) -> Vec<u32> {
    let mut permutation: Vec<u32> = (0..n as u32).collect();
    let mut rng = rand::thread_rng();

    for i in (1..n).rev() {
        let j = rng.gen_range(0..=i);
        permutation.swap(i as usize, j as usize);
    }
    permutation
}

/**
 * Generate a random pair of indices.
 *
 * @param n: The number of indices.
 * @return A random pair of indices.
 */
pub fn random_pair(n: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    let x1 = rng.gen_range(0..n);
    let x2 = (rng.gen_range(0..(n - 1)) + 1 + x1) % n;

    (x1, x2)
}