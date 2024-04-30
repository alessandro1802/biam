use std::fs::File;
use std::io::{self, BufRead};

use rand::prelude::*;

use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json;


// Struct to save the solution
#[derive(Serialize, Deserialize)]
struct Solution {
    best_distance: f32,
    best_solution: Vec<i64>,
    distances: Vec<f32>,
    runtimes: Vec<u128>,
    steps: Vec<u32>,
    evaluated: Vec<u32>,
}

/**
 * Save the solution to a file in the json format
 *
 * @param instance_name: Name of the instance
 * @param algorithm: Name of the algorithm
 * @param solutions: Vector of solutions
 * @param distances: Vector of distances
 * @param elapsed_time: Vector of elapsed times
 * @param steps: Vector of steps
 * @param evaluated: Vector of evaluated solutions
 */
pub fn save_solution(instance_name: &str, algorithm: &str, solutions: &Vec<Vec<u32>>, distances: &Vec<f32>, elapsed_time: &Vec<u128>, steps: &Vec<u32>, evaluated: &Vec<u32>) {
    // Schema
    // {
    //    "best_distance": 123.45,
    //    "best_solution": [1, 2, 3, 4, 5],
    //    "distances": [123.45, 123.45, 123.45],
    //    "runtimes": [123, 123, 123]
    //    "steps": [123, 123, 123]
    //    "evaluated": [123, 123, 123]
    // }
    let index_of_min_dist = distances.iter().enumerate().min_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(index, _)| index);
    std::fs::create_dir_all(format!("results/{}", instance_name)).unwrap();
    let file_path = format!("results/{}/{}.json", instance_name, algorithm);
    let mut file = std::fs::File::create(file_path).unwrap();

    // Write the best distance
    let best_distance = distances[index_of_min_dist.unwrap()];
    let best_solution = &solutions[index_of_min_dist.unwrap()];
    let best_solution = best_solution.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
    let best_solution = format!("[{}]", best_solution);
    let best_distance = format!("{:}", best_distance);
    let best_distance = best_distance.parse::<f32>().unwrap();
    let best_solution = serde_json::from_str(&best_solution).unwrap();
    let data = Solution {
        best_distance,
        best_solution,
        distances: distances.clone(),
        runtimes: elapsed_time.clone(),
        steps: steps.clone(),
        evaluated: evaluated.clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}


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
    * Calculate Intra-route delta
    *
    * @param i: The first node of the first edge
    * @param next_i: The second node of the first edge
    * @param j: The first node of the second edge
    * @param next_j: The second node of the second edge
    * @return: The delta fitness
    */
pub fn get_delta_intra_route(distance_matrix: &[Vec<f32>], i: u32, next_i: u32, j: u32, next_j: u32) -> f32 {
    distance_matrix[i as usize][j as usize] + distance_matrix[next_i as usize][next_j as usize]
    - (distance_matrix[i as usize][next_i as usize] + distance_matrix[j as usize][next_j as usize])
}

/**
* Swap 2 edges
*
* @param current_tour: The current tour
* @param next_i: The second node index of the first edge
* @param j: The first node index of the second edge
* @param best_tour: The best tour found
* @return: The modified best tour found
*/
pub fn swap_2_edges(current_tour: &[u32], next_i: usize, j: usize, mut best_tour: Vec<u32>) -> Vec<u32> {
    // Extract the slice [:next_i)
    let first_part = current_tour[..next_i].to_vec();
    // Extract and reverse the slice [next_i, (j + 1))
    let mut middle_part_rev = current_tour[next_i..=j].to_vec();
    middle_part_rev.reverse();
    // Extract the slice [(j + 1):]
    let last_part = current_tour[(j + 1)..].to_vec();
    // Combine the slices
    best_tour.clear();
    best_tour.extend_from_slice(&first_part);
    best_tour.extend_from_slice(&middle_part_rev);
    best_tour.extend_from_slice(&last_part);

    best_tour
}


/**
 * Generate a random permutation of range 0 to n-1.
 *
 * @param n: Permutation size.
 * @return A random permuatation.
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