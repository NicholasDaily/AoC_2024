use std::{collections::HashMap, env, io::{BufRead, Read}, iter::Map};
use std::str::FromStr;


fn main() {
    let mut input = Vec::<String>::new();
    if let Some(file_path)  = env::args().skip(1).next() {
        input = read_file(std::path::Path::new(&file_path));
    } else {
        input = 
            vec!["7 6 4 2 1".to_string(),
                "1 2 7 8 9".to_string(),
                "9 7 6 2 1".to_string(),
                "1 3 2 4 5".to_string(),
                "8 6 4 4 1".to_string(),
                "1 3 6 7 9".to_string()];
                input = vec!["38 39 40 43 42 45 50".to_string()];
    }
    
    let reports = parse_raw_input(input);

    println!("Safe:{}", count_safe_reports(&reports, 1));
    println!("Total:{}", reports.len())
   
}

fn count_safe_reports(reports : &Vec<Vec<i32>>, unsafe_tollerance : usize) -> usize {
    let mut count : usize = 0;
    for report in reports {
        
        if is_safe(&report, unsafe_tollerance) {
            count += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut report= report.clone();
            report.remove(i);
            if is_safe(&report, unsafe_tollerance) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn is_safe(report : &[i32], unsafe_tollerance : usize) -> bool{
    //same trend throughout
    //difference below 3
    let mut previous = report[0];
    let mut current = report[1];
    let trend= (current - previous).is_positive();
    let mut difference;
    let mut current_trend;

    for i in 2..report.len() {
        current_trend = (current - previous).is_positive();
        difference = (current - previous).abs();

        if difference > 3 || difference == 0{
            return false;
        }
        if current_trend != trend {
            return false;
        }
        previous = current;
        current = report[i];
    }
    current_trend = (current - previous).is_positive();
    difference = (current - previous).abs();

    if difference > 3 || difference == 0{
        return false;
    }
    if current_trend != trend {
        return false;
    }
    true
}



fn parse_raw_input(input : Vec<String>) -> Vec<Vec<i32>> {
    let mut parsed_input : Vec<Vec<i32>> = Vec::with_capacity(input.len());
    for line in input {
        let line_elements : Vec<&str> = line.split_whitespace().collect();
        let mut report_data : Vec<i32> = Vec::with_capacity(line_elements.len());
        for element in line_elements {
            report_data.push(element.parse().unwrap());
        }
        parsed_input.push(report_data);
    }
    parsed_input
}

fn read_file(filename: &std::path::Path) -> Vec<String> {
    let mut f_reader = std::io::BufReader::new(std::fs::File::open(filename).unwrap());
    let mut result = Vec::<String>::new();

    for line in f_reader.lines() {
        let  line = line.unwrap();
        result.push(line);
    }
    result
}