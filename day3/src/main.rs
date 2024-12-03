use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let pattern = r"mul\(([0-9]*),([0-9*])\)";
    let re = Regex::new(&pattern).unwrap();

    let mut global_sum = 0;
    for mat in re.captures_iter(&contents) {
        let num1 = mat[1].parse::<i32>().expect("Invalid number");
        let num2 = mat[2].parse::<i32>().expect("Invalid number");


        println!("{} * {} = {}", num1, num2, num1 * num2);

        global_sum += num1 * num2;
    }

    println!("{}", global_sum);
}

