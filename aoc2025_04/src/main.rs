use std::fs;
use std::io;

fn count_accessible_rolls(grid: &[&str]) -> usize {
    let mut count = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                let neighbors = count_neighbors(grid, row, col);
                if neighbors < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_neighbors(grid: &[&str], row: usize, col: usize) -> usize {
    let mut count = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in directions.iter() {
        let new_row = (row as i32 + dr) as usize;
        let new_col = (col as i32 + dc) as usize;
        if new_row < grid.len() && new_col < grid[new_row].len() {
            if grid[new_row].chars().nth(new_col) == Some('@') {
                count += 1;
            }
        }
    }
    
    count
}

fn print_grid_with_accessible(grid: &[&str]) {
    println!("\nGrid with accessible paper rolls (marked with 'x'):");
    for (row, line) in grid.iter().enumerate() {
        let mut result = String::new();
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                let neighbors = count_neighbors(grid, row, col);
                if neighbors < 4 {
                    result.push('x');
                } else {
                    result.push('@');
                }
            } else {
                result.push(ch);
            }
        }
        println!("{}", result);
    }
}

fn main() -> io::Result<()> {
    let content = fs::read_to_string("input.txt")?;
    let grid: Vec<&str> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    if grid.is_empty() {
        eprintln!("Error: input.txt is empty or not existing!");
        return Ok(());
    }
    let accessible_count = count_accessible_rolls(&grid);
    print_grid_with_accessible(&grid);
    println!("Number of accessible paper rolls: {}", accessible_count);
    Ok(())
}

