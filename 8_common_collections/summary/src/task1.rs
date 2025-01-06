use std::collections::HashMap;

fn calculate_median(arr: &[i32]) -> f32 {
    // create vector
    let mut v = Vec::new();
    for i in arr {
        v.push(i);
    }

    // sort vector
    v.sort();

    // calculate median
    let mut median: f32 = 0.0;
    let median_index = v.len() / 2;

    match v.len() % 2 {
        0 => median = (*v[median_index] + *v[median_index - 1]) as f32 / 2.0,
        1 => median = *v[median_index] as f32,
        _ => (),
    }

    return median;
}

fn calculate_mode(arr: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut mode: Vec<i32> = Vec::new();
    let mut max_count = 0;
    for i in arr {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
        }
    }
    for (k, v) in map {
        if v == max_count {
            mode.push(*k);
        }
    }
    return mode;
}

pub fn task1_print(arr: &[i32]) {
    println!("array = {:?} \nmedian = {} mode = {:?}\n", arr, calculate_median(arr), calculate_mode(arr));
}