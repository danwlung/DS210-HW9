//Daniel Lung
//Question 1
//Collaborators: None

use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::io::Write;

#[derive(Debug)]
struct Point {
    //Creates point struct that contains the value and the label
    x: i32,
    label: i32
}

fn gen_rand_points() -> Point {
    //Creates points with the random value and label
    let x = rand::thread_rng().gen_range(-1000000000..1000000000);
    let label = rand::thread_rng().gen_range(0..2);

    Point {x, label}
}

fn gen_file(path: &str, num_points: usize) {
    //Create file, adds the points to it
    let mut file = File::create(path).expect("Unable to create file");

    for _i in 0..num_points {
        //Generate the random points
        let point = gen_rand_points();

        //Create a string of x and label from the point, then "append" it to the file
        let s: String = format!("{} {}\n", point.x, point.label);
        file.write_all(s.as_bytes()).expect("Unable to create file");
    }
}

fn read_file(path: &str) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();

    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<i32>().unwrap();
        let y = v[1].parse::<i32>().unwrap();
        result.push((x, y));
    }
    return result;
}

fn find_best(mut txt_file : Vec<(i32,i32)>) -> (i32, i32, i32, f32) {
    txt_file.sort_by(|x,label| x.partial_cmp(label).unwrap());
    let length = txt_file.len();

    //keep track of error values
    let mut err_val = length as i32;
    
    //keep track of index for best split
    let mut best_index = 0;

    for i in 0..length {
        //Count errors on the left side of the split
        let mut err_val_left = 0;
        for i_left in 0..i {
            if txt_file[i_left].1 == 0 {
                err_val_left+=1;
            }
        }

        //Count errors on the right side of the split
        let mut err_val_right = 0;
        for i_right in i..length {
            if txt_file[i_right].1 == 1 {
                err_val_right+=1;
            }
        }

        //Store the errors to compare
        let err_val2 = length as i32 - (err_val_left + err_val_right);

        //Compares the number of errors to find the one with less errors and save it
        if err_val2 < err_val {
            err_val = err_val2;
            best_index = i;
        }
    }

    // Calculate for the accuracy
    let accuracy = 1.0 - (err_val as f32) / (length as f32);

    let x_value = txt_file[best_index].0;

    (best_index as i32, x_value, err_val, accuracy)
}

fn main() {
    let path = "data.txt";
    let num_points = 100;
    gen_file(path, num_points);
    println!("File '{}' created successfully with random points.", path);

    let data = read_file(path); 
    let result = find_best(data);

    println!("Best split index: {}", result.0);
    println!("Best split where x: {}", result.1);
    println!("Total errors: {}", result.2);
    println!("Accuracy: {:.2}", result.3);
}
