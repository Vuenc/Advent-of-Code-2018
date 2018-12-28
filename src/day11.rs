use ndarray::prelude::*;
use std::ops::RangeInclusive;
use std::collections::VecDeque;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn calculate_power_level(x: usize, y: usize, serial_number: i32) -> i32 {
    ((((x as i32 + 10) * y as i32 + serial_number) * (x as i32 + 10)) / 100) % 10 - 5
}


fn get_max_window(serial_number: i32, window_sizes: RangeInclusive<usize>) -> (usize, usize, usize) {
    let mut fuel_cells = Array2::from_elem((WIDTH, HEIGHT), 0);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            fuel_cells[(x, y)] = calculate_power_level(x + 1, y + 1, serial_number);
        }
    }

    let mut last_n_square_sums = VecDeque::with_capacity(2);
    let mut max_windows = Vec::new();
    for n in window_sizes {

        let n_square_sums = {
            if last_n_square_sums.len() < 2 {
                calculate_window_sums(n, &fuel_cells)
            } else {
                let two_before = last_n_square_sums.pop_front().unwrap();
                calculate_window_sums_from_last(n, &fuel_cells, 
                    last_n_square_sums.back().unwrap(), &two_before)
        }};

        let ((max_x, max_y), &el) = n_square_sums.indexed_iter().max_by_key(|(_, &el)| el).unwrap();
        max_windows.push(((max_x, max_y), el, n));
        last_n_square_sums.push_back(n_square_sums);
    }
    let ((max_x, max_y), _, max_window_size) = max_windows.iter().max_by_key(|(_, el, _)| el).unwrap();
    (*max_x, *max_y, *max_window_size)
}

/// Calculate window sums by summing the values in each window (used for the base case)
fn calculate_window_sums(n: usize, fuel_cells: &Array2<i32>) -> Array2<i32> {
    let mut n_square_sums = Array2::from_elem((WIDTH - n + 1, HEIGHT - n + 1), 0_i32);       
    for base_x in 0..WIDTH - n + 1 {
        for base_y in 0..HEIGHT - n + 1 {
            for x in 0..n {
                for y in 0..n {
                    n_square_sums[(base_x, base_y)] += fuel_cells[(base_x + x, base_y + y)];
                }
            }
        }
    }
    n_square_sums
}

/// Use dynamic programming to calculate windows of certain size quickly based on the last two
/// arrays of window sums (used for later cases)
fn calculate_window_sums_from_last(n: usize, fuel_cells: &Array2<i32>, one_before: &Array2<i32>, 
    two_before: &Array2<i32>) -> Array2<i32> 
{
    let mut n_square_sums = Array2::from_elem((WIDTH - n + 1, HEIGHT - n + 1), 0_i32);
    for base_x in 0..WIDTH - n + 1 {
        for base_y in 0..HEIGHT - n + 1 {
            n_square_sums[(base_x, base_y)] = 
                one_before[(base_x, base_y)]
                + one_before[(base_x + 1, base_y + 1)] 
                - two_before[(base_x + 1, base_y + 1)] 
                + fuel_cells[(base_x, base_y + n - 1)]
                + fuel_cells[(base_x + n - 1, base_y)];
        }
    }
    n_square_sums
}

pub fn star1(line: &String) -> String {
    let serial_number = line.parse::<i32>().unwrap();
    let (max_x, max_y, _) = get_max_window(serial_number, 3..=3);

    format!("{},{}", max_x + 1, max_y + 1)
}

pub fn star2(line: &String) -> String {
    let serial_number = line.parse::<i32>().unwrap();
    let (max_x, max_y, window_size) = get_max_window(serial_number, 3..=300);

    format!("{},{},{}", max_x + 1, max_y + 1, window_size)
}