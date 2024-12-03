use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("puzzle.txt").unwrap();

    let pattern = r"mul\(([0-9]*),([0-9]*)\)|(do\(\))|(don't\(\))";
    let re = Regex::new(&pattern).unwrap();

    let mut can_do = true;
    let mut global_sum = 0;
    for mat in re.captures_iter(&contents) {

        if let Some(do_match) = mat.get(3) {
            if do_match.as_str() == "do()" {
                can_do = true;
                continue;
            }
        }
        if let Some(dont_match) = mat.get(4) {
            if dont_match.as_str() == "don't()" {
                can_do = false;
                continue;
            }
        }

        if can_do {
            let num1 = mat[1].parse::<i32>().expect("Invalid number");
            let num2 = mat[2].parse::<i32>().expect("Invalid number");

            global_sum += num1 * num2;
            println!("{}+{} = {}", num1, num2, num1 * num2);
        }
    }

    println!("{}", global_sum);

}

