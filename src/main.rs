use std::vec::Vec;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn quick_sort(data: &mut [i32], pivot_selection: fn(&[i32]) -> i32) -> i32 {
    //    println!("length of data: {}", data.len());
    let mut num_comparisons = 0; // record the number of comparisons made by partitioning
    if data.len() == 1 {
        return num_comparisons; 
    }
    let pivot_index: i32 = pivot_selection(data);
    let final_pivot_index: i32 = partition(&mut *data, pivot_index);
    num_comparisons += (data.len() as i32 - 1);
    /*
    println!("After partitioning");
    for elem in data.iter() {
        println!("{}", elem);
    }
    */ 
    let (left_portion, right_portion) = data.split_at_mut(final_pivot_index as usize);

    if right_portion.len() > 1 {
        if let Some((_first, real_right_portion)) = right_portion.split_first_mut() {
            /*
            println!("Right portion: ");
            for elem in real_right_portion.iter() {
                println!("{}", elem);
            }
            */
            num_comparisons += quick_sort(real_right_portion, pivot_selection); // don't recurse if the right portion is empty 
        }
    } else {
        num_comparisons += quick_sort(right_portion, pivot_selection);
    }
    /*
    println!("Left portion: ");
    for elem in left_portion.iter() {
        println!("{}", elem);
    }
    */
    if left_portion.len() > 0 {
        num_comparisons += quick_sort(left_portion, pivot_selection);
    }
    num_comparisons
}
/*
 * Three ways of selecting a pivot below
 */ 
fn first_element_pivot(data: &[i32]) -> i32 {
    0
}

fn last_element_pivot(data: &[i32]) -> i32 {
    (data.len() - 1) as i32
}

fn median_pivot(data: &[i32]) -> i32 {
    let first_elem = data[0];
    let mid_elem = data[data.len() / 2];
    let last_elem = data[data.len() - 1];
    let mut temp_vec: Vec<i32> = vec![first_elem, mid_elem, last_elem];
    temp_vec.sort();
    if temp_vec[1] == first_elem {
        0
    } else if temp_vec[1] == mid_elem {
        (data.len() / 2) as i32
    } else {
        (data.len() - 1) as i32
    }
}


fn partition(data: &mut [i32], pivot_index: i32) -> i32 {
    if pivot_index != 0 {
        data.swap(pivot_index as usize, 0);
    }
    //  println!("Pivot is {}", data[0]);
    let mut i = 1;
    let mut larger_element_seen = false;
    for j in 1..data.len() {
        if data[j] < data[0] {
            if larger_element_seen {
                data.swap(i, j);
            }
            i += 1;
        } else {
            larger_element_seen = true;
        }
    }
    if i as i32 != 1 {
        data.swap(0, i - 1);
        return (i - 1) as i32;
    }
    0
}



fn main() {
    let file = File::open("QuickSort.txt").unwrap();
    let mut data_vec: Vec<i32> = Vec::new();
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    let mut num_bytes = buf_reader.read_line(&mut line).expect("line reading error");
    while num_bytes > 0 {
        data_vec.push(line.trim().parse::<i32>().unwrap()); // trim off trailing newline characters 
        line.clear();
        num_bytes = buf_reader.read_line(&mut line).expect("line reading error");
    }
    for elem in data_vec.iter() {
        println!("{}", elem);
    }
    println!("{} integers read", data_vec.len());
    let comparisons = quick_sort(&mut data_vec[..], median_pivot);
    
    for elem in data_vec.iter() {
        println!("{}", elem);
    }

    println!("Number of comparisons: {}", comparisons);
}
