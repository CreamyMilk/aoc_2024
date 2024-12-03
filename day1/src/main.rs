use std::fs;
use std::collections::HashMap;
fn main() {
    let contents = fs::read_to_string("puzzle.txt").expect("Failed to read file");
    let lines = contents.split("\n");


    let mut left_arr:Vec<i32> = Vec::new();
    let mut right_arr:Vec<i32> = Vec::new();

    


    let mut global_sum = 0;
    for line in lines {
        let num_line: Vec<&str>= line.split_whitespace().collect();

        let num1:i32 = num_line[0].parse().expect("Failed to parse number");
        let num2:i32 = num_line[1].parse().expect("Failed to parse number");

        left_arr.push(num1);
        right_arr.push(num2);

        left_arr.sort();
        right_arr.sort();
    }

    for i in 0..left_arr.len(){
        let diff = (left_arr[i]-right_arr[i]).abs();

        println!(":: {} - {} = {}", left_arr[i], right_arr[i],diff);
        global_sum += diff;
    }
    println!("Global sum: {}", global_sum);

    // Part 2
    let mut counter_map:HashMap<i32,i32> = HashMap::new();

    // prefill keys
    for num in &left_arr{
        counter_map.insert(*num, 0);
    }

    for num in &right_arr{
        if counter_map.contains_key(&num){
            counter_map.insert(*num, counter_map[&num]+1);
        }
    }

    let mut similarity_prod = 0;
    for number_for_one in left_arr{
        similarity_prod += number_for_one * counter_map[&number_for_one];
    }
    
    println!("Similarity prod: {}", similarity_prod);

}


