use std::fs;

fn is_it_safe(report:&Vec<i32>)->bool{
    // true if increasing, false if decreasing
    let consitent_type = report[0] > report[1]; 

    for i in 0..report.len()-1 {
        if (consitent_type && report[i] < report[i+1]) || (!consitent_type && report[i] > report[i+1]) {
            return false;
        }

        let diff = (report[i] - report[i+1]).abs();

        if diff > 3 || diff < 1{
            return false;
        }
    }

    return true;
}

fn main() {
    let contents = fs::read_to_string("puzzle.txt").unwrap();
    let report_strings = contents.split("\n").collect::<Vec<&str>>();

    let mut reports:Vec<Vec<i32>> = Vec::new();

    for single_report in report_strings {
        let parsed_reports = single_report.split_whitespace().collect::<Vec<&str>>()
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

        reports.push(parsed_reports);
    }

    let mut safe_reports = 0;
    for report in reports {
        if is_it_safe(&report) {
            safe_reports += 1;
        }
    }

    println!("Number of safe reports: {}", safe_reports);
}



 
