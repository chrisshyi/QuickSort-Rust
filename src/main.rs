use std::vec::Vec;

fn quick_sort(data: &mut [i32]) {
    println!("length of data: {}", data.len());
    if data.len() == 1 || data.len() == 0 {
        return;
    }
    let pivot_index: i32 = choose_pivot(data);
    let final_pivot_index: i32 = partition(&mut *data, pivot_index);
    println!("After partitioning");
    for elem in data.iter() {
        println!("{}", elem);
    }
    
    let (left_portion, right_portion) = data.split_at_mut(final_pivot_index as usize);
    if let Some((_first, real_right_portion)) = right_portion.split_first_mut() {
        println!("Right portion: ");
        for elem in real_right_portion.iter() {
            println!("{}", elem);
        }
        quick_sort(real_right_portion); // don't recurse if the right portion is empty 
    }

    println!("Left portion: ");
    for elem in left_portion.iter() {
        println!("{}", elem);
    }
    quick_sort(left_portion);
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
    let mut data_vec = vec![60, 10, 2, 35, 24];
    for elem in data_vec.iter() {
        println!("{}", elem);
    }
    quick_sort(&mut data_vec[..]);

    for elem in data_vec.iter() {
        println!("{}", elem);
    }
}




