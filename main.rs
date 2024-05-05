mod parser;
mod bfs;
mod statistics;

use parser::parse_csv;
use bfs::bfs;
use statistics::{degree_distribution, mean, standard_deviation, mode, variance, count_connected_components, calculate_density};

fn main() {
    let file_path = "src/euroroad.csv";// Specify the file path of the csv file to read

    let graph = parse_csv(file_path).expect("Failed to parse CSV");
    // println!("Graph: {:?}", graph); // Debugging purpose: Print the graph to verify its structure

    // Prove that bfs works
    let start_vertex = "316";
    let goal_vertex = "330";

    // Degree Distribution
    let degrees = degree_distribution(&graph);
    println!("Degree Distribution: {:?}", degrees);

    // Printing the decisions based on the input start vertex and goal vertex
    match bfs(&graph, start_vertex, goal_vertex) {
        Some(distance) => println!("Distance between {} and {} is {}", start_vertex, goal_vertex, distance),
        None => println!("No path found between {} and {}", start_vertex, goal_vertex),
    }
   
    // Finding the mean of the dataset
    let mean_val = mean(&degrees);
    println!("Mean degree: {}", mean_val);

    // Finding the variance of the dataset
    let variance_val = variance(&degrees, mean_val);
    println!("Variance: {}", variance_val);

    // Finding the standard deviation of the dataset
    let std_deviation = standard_deviation(&degrees, mean_val);
    println!("Standard Deviation: {}", std_deviation);

    // Findind the mode of the dataset
    let mode_val = mode(&degrees);
    println!("Mode: {:?}", mode_val);

    // Connected Components
    let components = count_connected_components(&graph);
    println!("Number of Connected Components: {}", components);

    // Graph Density
    let density = calculate_density(&graph);
    println!("Graph Density: {}", density);
}