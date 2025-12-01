#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Dial {
    dialed_number: u32,
    count_point_zero: u32,
    line_number: u32
}

impl Dial {
    fn right(&mut self, count: u32) {
        let n = self.dialed_number + (count - 100 * (count / 100));
        self.dialed_number = if n >= 100 { n - 100 } else { n };
        if self.dialed_number == 0 {
            self.count_point_zero += 1;
        }
        println!("{}. Rechts {}: {}", self.line_number, count, self.dialed_number)
    }

    fn left(&mut self, count: u32) {
        let n = (self.dialed_number as i32) - (count - 100 * (count / 100)) as i32;
        self.dialed_number = if n < 0 { (100 + n) as u32 } else { n as u32 };
        if self.dialed_number == 0 {
            self.count_point_zero += 1;
        }
        println!("{}. Links {}: {}", self.line_number, count, self.dialed_number);
    }

    fn split_string(&self, s: &str) -> Result<(char, u32), String> {
        let mut chars = s.chars();
        let letter = chars.next().ok_or("No letter")?;
        let number_str = chars.collect::<String>();
        let number = number_str
            .parse()
            .map_err(|_| format!("Failed to parse '{}' as u32", number_str))?;
        Ok((letter, number))
    }

    fn command(&mut self, s: &str) {
        self.line_number += 1;
        match self.split_string(s) {
            Ok((direction, count)) => match direction {
                'L' => self.left(count),
                'R' => self.right(count),
                _ => {}
            },
            Err(e) => println!("Error: {e}"),
        }
    }

    fn new() -> Self {
        Dial {
            dialed_number: 50,
            count_point_zero: 0,
            line_number: 1
        }
    }
}

fn read_from_file(filename: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dial = Dial::new();
    let filename = "input.txt";
    let reader: BufReader<File> = read_from_file(filename).unwrap();
    for line in reader.lines() {
        let line = line?;
        dial.command(&line);
    }
    println!("Result: {}", dial.count_point_zero);
    Ok(())
}
