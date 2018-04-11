use std::vec::Vec;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn quick_sort(data: &mut [i32]) {
    println!("length of data: {}", data.len());
    if data.len() == 1 {
        return;
    }
    let pivot_index: i32 = choose_pivot(data);
    let final_pivot_index: i32 = partition(&mut *data, pivot_index);
    println!("After partitioning");
    for elem in data.iter() {
        println!("{}", elem);
    }
    
    let (left_portion, right_portion) = data.split_at_mut(final_pivot_index as usize);

    if right_portion.len() > 1 {
        if let Some((_first, real_right_portion)) = right_portion.split_first_mut() {
            println!("Right portion: ");
            for elem in real_right_portion.iter() {
                println!("{}", elem);
            }
            quick_sort(real_right_portion); // don't recurse if the right portion is empty 
        }
    } else {
        quick_sort(right_portion);
    }
    println!("Left portion: ");
    for elem in left_portion.iter() {
        println!("{}", elem);
    }
    if left_portion.len() > 0 {
        quick_sort(left_portion);
    }
}

fn choose_pivot(data: &[i32]) -> i32 {
    0
}

fn partition<'a>(data: &'a mut [i32], pivot_index: i32) -> i32 {
    if pivot_index != 0 {
        data.swap(pivot_index as usize, 0);
    }
    println!("Pivot is {}", data[0]);
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
    /*
    let mut data_vec = vec![3, 20, 15, 1, 699, 342, 360, 134, 10348, 10, 13];
    for elem in data_vec.iter() {
        println!("{}", elem);
    }
    quick_sort(&mut data_vec[..]);

    for elem in data_vec.iter() {
        println!("{}", elem);
    }
    */
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
}
