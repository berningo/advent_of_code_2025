use std::fs::File;
use std::io::{BufRead, BufReader};

mod duplicate_detector {
    use std::num::ParseIntError;
    use std::io::ErrorKind;

    #[derive(Debug)]
    pub struct ProductIdRanges {
        start_id: u64,
        end_id: u64,
        sum: u64
    }

    impl ProductIdRanges {
        pub fn new(s: &str) -> Result<Self, std::io::Error> {
            let numbers_result: Result<Vec<u64>, ParseIntError> = s.split('-')
                .map(|x| x.parse::<u64>())
                .collect();
            let numbers = numbers_result.map_err(
                |e| std::io::Error::new(ErrorKind::InvalidData, e)
            )?;
            Ok(Self {
                start_id: numbers[0],
                end_id: numbers[1],
                sum: 0
            })
        }
        
        fn count_digits(&self, n: u64) -> u64 {
            n.to_string().len() as u64
        }
        
        fn is_each_half_string_same(&self, n: u64) -> bool {
            let s = n.to_string();
            let (first_half, second_half) = s.split_at(s.len() / 2);
            first_half == second_half
        }
        
        pub fn sum_of_invalid_ids_in_range(&self) -> u64 {
            self.sum
        }

        pub fn check(&mut self) {
            for num in self.start_id..=self.end_id {
                if self.count_digits(num) % 2 == 0 && self.is_each_half_string_same(num) {
                    println!("\tInvalid ID: {}", num);
                    self.sum += num;
                }
            }
        }
    }
}

use duplicate_detector::ProductIdRanges;

fn read_line(filename: &str) -> Result<String, std::io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line = read_line("input.txt").unwrap();
    let mut total_sum = 0;
    for range_string in line.split(",") {
        println!("Product ID Ranges: {}", range_string);
        let mut product_id_range = ProductIdRanges::new(range_string)?;
        product_id_range.check();
        total_sum += product_id_range.sum_of_invalid_ids_in_range();
    }
    println!("Sum is: {}", total_sum);
    Ok(())
}
