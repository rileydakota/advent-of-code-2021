use std::fs;

fn main() {
    part_1();
    part_2();
}

trait Navigable {
    fn up(&mut self, amount: i32);
    fn down(&mut self, amount: i32);
    fn forward(&mut self, amount: i32);
    fn print_current_pos(&self);
}
struct Position {
    depth: i32,
    horizontal: i32,
}

impl Navigable for Position {
    fn up(&mut self, amount: i32) {
        self.depth -= amount
    }

    fn down(&mut self, amount: i32) {
        self.depth += amount
    }

    fn forward(&mut self, amount: i32) {
        self.horizontal += amount
    }

    fn print_current_pos(&self) {
        println!(
            "current depth is {} and current horizontal distance is {}",
            self.depth, self.horizontal
        )
    }
}

fn part_1() {
    println!("Advent of Code 2021 - Day 2 Part 1!");

    let filename = "../../data/day_2.txt";
    let load_statement = format!("Loading problem data from {}", filename);

    let mut pos = Position {
        depth: 0,
        horizontal: 0,
    };

    println!("{}", load_statement);

    let problem_data = fs::read_to_string(filename).expect("loading of file failed!");

    let split = problem_data.split("\n");

    for direction in split {
        let dir_split: Vec<String> = direction
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        if dir_split[0] == "forward" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.forward(int);
        } else if dir_split[0] == "up" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.up(int)
        } else if dir_split[0] == "down" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.down(int)
        }
    }
    pos.print_current_pos();
    println!("Answer is {}", pos.depth * pos.horizontal)
}

struct PositionAim {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Navigable for PositionAim {
    fn up(&mut self, amount: i32) {
        self.aim -= amount
    }

    fn down(&mut self, amount: i32) {
        self.aim += amount
    }

    fn forward(&mut self, amount: i32) {
        self.horizontal += amount;
        self.depth += self.aim * amount;
    }

    fn print_current_pos(&self) {
        println!(
            "current depth is {} and current horizontal distance is {}",
            self.depth, self.horizontal
        )
    }
}

fn part_2() {
    println!("Advent of Code 2021 - Day 2 Part 2!");

    let filename = "../../data/day_2.txt";
    let load_statement = format!("Loading problem data from {}", filename);

    let mut pos = PositionAim {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    println!("{}", load_statement);

    let problem_data = fs::read_to_string(filename).expect("loading of file failed!");

    let split = problem_data.split("\n");

    for direction in split {
        let dir_split: Vec<String> = direction
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        if dir_split[0] == "forward" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.forward(int);
        } else if dir_split[0] == "up" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.up(int);
        } else if dir_split[0] == "down" {
            let int: i32 = dir_split[1].parse().unwrap();
            pos.down(int);
        }
    }
    pos.print_current_pos();
    println!("Answer is {}", pos.depth * pos.horizontal)
}
