use std::fs;

fn main() {
    part_1();
}

#[derive(Debug)]
struct commonBits {
    zero_bits: u32,
    one_bits: u32,
}

//12 digits
#[derive(Debug)]
struct diagnosticReport {
    bits: Vec<commonBits>,
}

fn get_answer(data: diagnosticReport) {
    let mut least_cmn_str_array = Vec::new();
    let mut most_cmn_str_array = Vec::new();
    for bits in data.bits {
        most_cmn_str_array.push(get_most_common_bit(&bits));
        least_cmn_str_array.push(get_least_common_bit(&bits));
    }

    let binary_string_least = most_cmn_str_array.join("");
    println!("{} is the gamma bits", binary_string_least);
    let base10_gamma = isize::from_str_radix(&binary_string_least, 2).unwrap();
    println!("{} is the gamma rate ", base10_gamma);
    let binary_string_most = least_cmn_str_array.join("");
    println!("{} is the epsilon bits", binary_string_most);
    let base10_epsilon = isize::from_str_radix(&binary_string_most, 2).unwrap();
    println!("{} is the epsilon rate ", base10_epsilon);
    println!("{} is the answer!", base10_epsilon * base10_gamma);
}

fn get_most_common_bit(bit_count: &commonBits) -> &'static str {
    if bit_count.one_bits > bit_count.zero_bits {
        return "1";
    } else {
        return "0";
    }
}

fn get_least_common_bit(bit_count: &commonBits) -> &'static str {
    if bit_count.one_bits < bit_count.zero_bits {
        return "1";
    } else {
        return "0";
    }
}

fn part_1() {
    println!("Advent of Code 2021 - Day 3 Part 1!");

    let filename = "../../data/day_3.txt";

    let problem_data = fs::read_to_string(filename).expect("loading of file failed!");

    let split = problem_data.split("\n");
    let mut diagnostic_report = diagnosticReport { bits: vec![] };
    for _ in 0..12 {
        diagnostic_report.bits.push({
            commonBits {
                zero_bits: 0,
                one_bits: 0,
            }
        })
    }
    for binary_line in split {
        //println!("{}", binary_line);
        //let mut binary_line_bits: Vec<char>= binary_line.chars().collect();
        let mut binary_line_bits: Vec<&str> = binary_line.split("").collect();
        binary_line_bits.remove(0);
        binary_line_bits.remove(12);
        let mut count = 0;
        for bit in binary_line_bits {
            match bit {
                "0" => diagnostic_report.bits[count].zero_bits += 1,
                "1" => diagnostic_report.bits[count].one_bits += 1,
                _ => {
                    println!("found unknown character {} - fatal error", bit);
                    panic!();
                }
            };
            count += 1;
        }
    }
    println!("{:?}", diagnostic_report);

    get_answer(diagnostic_report);
}

//assert!("1", get_most_common_bit({zero_bits:15, one_bits:22}));
//assert!("0", get_most_common_bit({zero_bits:22, one_bits:11}));
