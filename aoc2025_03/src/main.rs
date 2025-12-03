use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_highest_joltage(line: &str) -> u64 {
    let mut highest_joltage = 0;
    for (index, char1) in line.chars().enumerate() {
        for char2 in line.chars().skip(index+1) {
            let number = ((char1 as u64 - '0' as u64) * 10) + (char2 as u64 - '0' as u64);
            highest_joltage = if number > highest_joltage { number } else { highest_joltage };
        }
    }
    highest_joltage
}

fn read_from_file(filename: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "input.txt";
    let reader: BufReader<File> = read_from_file(filename)?;
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        println!("Line: {}", line);
        let highest_joltage = get_highest_joltage(&line);
        sum += highest_joltage;
        println!("Highest Joltage: {}", highest_joltage);
    }
    println!("Result: {}", sum);
    Ok(())
}
