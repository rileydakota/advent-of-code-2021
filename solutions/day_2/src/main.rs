use std::fs;

fn main() {
    part_1()
}

struct Position {
    depth: i32,
    horizontal: i32
}

impl Position {
    fn up(&mut self, amount: i32){
        self.depth -= amount
    }

    fn down(&mut self, amount: i32){
        self.depth += amount
    }

    fn forward(&mut self, amount: i32){
        self.horizontal += amount
    }

    fn print_current_pos(&self){
        println!("current depth is {} and current horizontal distance is {}", self.depth, self.horizontal)
    }
}

fn part_1() {
    println!("Advent of Code 2021 - Day 2 Part 1!");
    
    let filename = "../../data/day_2.txt";
    let load_statement = format!("Loading problem data from {}", filename);
    
    let mut pos = Position { depth: 0, horizontal: 0 };

    println!("{}", load_statement);

    let problem_data = fs::read_to_string(filename)
        .expect("loading of file failed!");
    
    let split = problem_data.split("\n");

    for direction in split {
        println!("{}", direction);
        let dir_split: Vec<String> = direction.split_whitespace().map(|s| s.to_string()).collect();
        if dir_split[0] == "forward" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.forward(int);
        }
        else if dir_split[0] == "up" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.up(int)
        }
        else if dir_split[0] == "down" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.down(int)
        }
    }
    pos.print_current_pos();
    println!("Answer is {}", pos.depth * pos.horizontal)
}