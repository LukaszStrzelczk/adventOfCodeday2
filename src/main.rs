use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

fn read_file(path: &str) -> Vec<Vec<i32>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Unable to open a file: {}, erorr: {}", path, e);
            exit(1);
        }
    };
    let buffer = BufReader::new(file);
    let mut input: Vec<Vec<i32>> = Vec::new();
    for line in buffer.lines() {
        match line {
            Ok(line) => input.push(line.split(' ').map(|x| x.parse().unwrap()).collect()),
            Err(e) => {
                println!("Error while reading a line: {}", e);
                exit(2)
            }
        }
    }
    input
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports_count = 0;
    for report in input {
        let mut descending = true;
        let mut ascending = true;
        let report_size = report.len();
        for i in 0..report_size - 1 {
            let level_diffrence=report[i]-report[i+1];
            if level_diffrence.abs()<1||level_diffrence.abs()>3{
                descending=false;
                ascending=false;
                break;
            }
            match level_diffrence>0{
                true=>ascending=false,
                false=>descending=false
            }
            if !ascending&&!descending{break}
        }
        safe_reports_count=if ascending||descending {safe_reports_count+1}else{safe_reports_count};
    }

    safe_reports_count
}
fn main() {
    let path = "input.txt";
    let input = read_file(path);
    println!("result: {}",part1(input));
}
