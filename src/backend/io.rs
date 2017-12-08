/*
 * Author: Samuel Resendez
 * This stores all of the functions and utilities for handling file systems, as well as keeping
 * and storing intermediate JSON data while parsing 
 */


use std::fs;
use std::fs::File;


/*
 * save_graph is a function that is trying to save a graph to the grust_build directory, so that its
 * contents can then be constructed into a visualization by the frontend
 */





/*
 * From Here on out it's non public functions; these are just utilities to make it easier
 * to implement the public interface
 */
fn create_file(name: &str) -> File {
    let _ = fs::remove_file(name);
    fs::File::create(name).unwrap()
}



