/*
 * Author: Samuel Resendez
 * This stores all of the functions and utilities for handling file systems, as well as keeping
 * and storing intermediate JSON data while parsing 
 */


use std::fs;
use std::fs::File;
use std::io::{Write, Result};
use serde_json;

use backend::graph::Graph;

/*
 * save_graph is a function that is trying to save a graph to the grust_build directory, so that its
 * contents can then be constructed into a visualization by the frontend
 */
pub fn save_graph(graph: &Graph) -> Result<usize> {
    let serialized = serde_json::to_string(graph).unwrap();
    let _ = fs::create_dir("grust_build/");
    let file_path = format!("grust_build/{}.json", graph.description);
    let mut file = create_file(&file_path);
    file.write(serialized.as_bytes())

}




/*
 * From Here on out it's non public functions; these are just utilities to make it easier
 * to implement the public interface
 */
fn create_file(name: &str) -> File {
    let _ = fs::remove_file(name);
    fs::File::create(name).unwrap()
}



