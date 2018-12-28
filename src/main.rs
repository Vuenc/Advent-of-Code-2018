extern crate ndarray;
extern crate regex;
extern crate bit_vec;
extern crate elapsed;
extern crate intrusive_collections;
extern crate linked_list;
#[macro_use] extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use crate::InputMode::*;
use elapsed::{measure_time, ElapsedDuration};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

enum InputMode {
    Lines(Option<fn(&Vec<String>) -> String>, Option<fn(&Vec<String>) -> String>),
    OneLine(Option<fn(&String) -> String>, Option<fn(&String) -> String>)
}

static INPUT_DIR: &str = &"inputs/";
static DAYS: &'static [InputMode]  = &[
    Lines(None, Some(day1::star2)),
    Lines(Some(day2::star1), Some(day2::star2)),
    Lines(Some(day3::star1), Some(day3::star2)),
    Lines(Some(day4::star1), Some(day4::star2)),
    OneLine(Some(day5::star1), Some(day5::star2)),
    Lines(Some(day6::star1), Some(day6::star2)),
    Lines(Some(day7::star1), Some(day7::star2)),
    OneLine(Some(day8::star1), Some(day8::star2)),
    Lines(Some(day9::star1), Some(day9::star2)),
    Lines(Some(day10::star1), Some(day10::star2)),
    OneLine(Some(day11::star1), Some(day11::star2)),
];

fn main() {
    run_day(11, None);
}

fn run_day(day: usize, star_constraint: Option<i32>) {
    let file = File::open(format!("{}{}{}{}", INPUT_DIR, "day", day, ".txt")).expect("Input file not found!");

    println!("Running Day {}:", day);
    let solution = match DAYS[day-1] {
        Lines(star1, star2) => {
            let lines = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
            execute_with_input(star1, star2, &lines, star_constraint)
        },
        OneLine(star1, star2) => { 
            let mut line = String::new();
            BufReader::new(file).read_line(&mut line).unwrap();
            execute_with_input(star1, star2, &line, star_constraint)
        }
    };
    if let Some((elapsed, solution1)) = solution.0 {
        println!("Star 1: {} ({})", solution1, elapsed);
    }
    if let Some((elapsed, solution2)) = solution.1 {
        println!("Star 2: {} ({})", solution2, elapsed);
    }
}

fn execute_with_input<F, T>(star1: Option<F>, star2: Option<F>, input: &T, star_constraint: Option<i32>) -> (Option<(ElapsedDuration, String)>, Option<(ElapsedDuration, String)>)
    where F: Fn(&T) -> String
{
    let (execute_star1, execute_star2) = match star_constraint {
        Some(1) => (true, false),
        Some(2) => (false, true),
        Some(_) => panic!("Invalid star constraint!"),
        None => (true, true)
    };
    let star1_result = star1.and_then(|day_func| {
        if execute_star1 {
            Some(measure_time(|| day_func(input)))
        } else {
            None
        }
    });
    let star2_result = star2.and_then(|day_func| { 
        if execute_star2 { 
            Some(measure_time(|| day_func(input)))
        } else { 
            None
        }
    });
    (star1_result, star2_result)
}