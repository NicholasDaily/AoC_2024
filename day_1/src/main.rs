use std::{collections::HashMap, env, io::{BufRead, Read}, iter::Map};
use std::str::FromStr;

fn main() {
    let mut input = Vec::<String>::new();
    if let Some(file_path)  = env::args().skip(1).next() {
        input = read_file(std::path::Path::new(&file_path));
    } else {
        input = 
            vec!["3   4".to_string(),
                "4   3".to_string(),
                "2   5".to_string(),
                "1   3".to_string(),
                "3   9".to_string(),
                "3   3".to_string()];
    }

    let  (input_a, input_b) = process_raw_input(input);
    
    let input_a = sort(&input_a);
    let input_b = sort(&input_b);
    //let result_vec = find_difference(input_a, input_b); //solution for part 1
    let result_vec = find_similarity_score(input_a, input_b);
    
   
    println!("{:?}", result_vec.iter().sum::<u32>());
}

fn find_similarity_score(input_a : Vec<u32>, input_b : Vec<u32>) -> Vec<u32> {
    let mut result : Vec<u32> = Vec::with_capacity(input_a.len());
    let mut map : HashMap<u32, u32> = HashMap::new();
    for num in input_a {
        if(map.contains_key(&num)) {
            result.push(*map.get(&num).unwrap());
        } else {
            if let Some(mut start_index) = find_first_occurrance(&input_b, num) {
                let mut counter = 0;
                while num == input_b[start_index] {
                    start_index += 1;
                    counter += 1;
                }
                map.insert(num, num * counter);
                result.push(num * counter);
            }else{
                map.insert(num, 0);
                result.push(0);
            }
        }
    }

    result
}

fn find_first_occurrance(input : &[u32], target : u32) -> Option<usize>{
    if let  Ok(mut start_index) = input.binary_search(&target) {
        let mut value_at_index = input[start_index];
        loop {
            if start_index == 0 {
                break;
            }
            if value_at_index == input[start_index - 1] {
                start_index -= 1;
            } else {
                break;
            }
        };
        Some(start_index)
    } else {
        None
    }

}


fn find_difference(input_a : Vec<u32>, input_b : Vec<u32>) -> Vec<u32> {
    let mut result : Vec<u32> = Vec::with_capacity(input_a.len());
    
    for i in 0..input_a.len() {
        let num : u32;
        if input_a[i] < input_b[i] {
            num = input_b[i] - input_a[i];
        }else{
            num = input_a[i] - input_b[i];
        }
        result.push(num)
    }

    result
}

fn process_raw_input(input : Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut result_a = Vec::<u32>::with_capacity(input.len());
    let mut result_b = Vec::<u32>::with_capacity(input.len());

    for line in input {
        let lines : Vec<&str> = line.split_whitespace().collect();
        result_a.push(lines[0].parse().unwrap());
        result_b.push(lines[1].parse().unwrap());
    }

    (result_a, result_b)
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

fn sort(list : &[u32]) -> Vec<u32>{
    if list.len() <= 1 {
        return Vec::from(list)
    } else {
        let sublist_a;
        let sublist_b;
        if list.len() % 2 == 0 {
            sublist_a = &list[..list.len()/2];
            sublist_b = &list[list.len()/2..];
        } else {
            sublist_a = &list[..list.len()/2 + 1];
            sublist_b = &list[list.len()/2 + 1..];
        }
        let sublist_a = &sort(sublist_a);
        let sublist_b = &sort(sublist_b);
        sort(sublist_b);
        let mut result = Vec::<u32>::with_capacity(sublist_a.len() + sublist_b.len());
        let mut index_a = 0;
        let mut index_b = 0;

        while index_a < sublist_a.len() && index_b < sublist_b.len() {
            if sublist_a[index_a] < sublist_b[index_b] {
                result.push(sublist_a[index_a]);
                index_a += 1;
            } else {
                result.push(sublist_b[index_b]);
                index_b += 1;
            }
        }

        while index_a < sublist_a.len() {
            result.push(sublist_a[index_a]);
            index_a += 1;
        }

        while index_b < sublist_b.len() {
            result.push(sublist_b[index_b]);
            index_b += 1;
        }
        

        result
    }

}