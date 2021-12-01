use std::fs;

fn main() {

    println!("Advent of Code 2021 - Day 1 Part 1!");
    
    let filename = "../../data/day_1.txt";
    let load_statement = format!("Loading problem data from {}", filename);
    
    println!("{}", load_statement);

    let problem_data = fs::read_to_string(filename)
        .expect("loading of file failed!");
    
        // println!("{}", problem_data)

    let mut increase_cnt = 0;
    let split = problem_data.split("\n");
    let mut prev = 0;
    
    for depth in split {
        let int: i32 = depth.parse().unwrap();
        if prev == 0 {
            prev = int;
            println!("{} (N/A - no previous measurement)", int);
            continue;
        } else if int > prev {
            increase_cnt += 1;
            prev = int;
            println!("{} (increased)", int);
        } else {
            println!("{} (decresed)", int);
            prev = int;
        }
    }
    let answer = increase_cnt.to_string();
    println!("{} is the answer!", answer);

}
