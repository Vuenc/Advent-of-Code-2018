use ndarray::prelude::*;
use std::cmp::{max};
use regex::Regex;
use self::SquareState::*;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SquareState {
    Untaken,
    TakenOnce,
    TakenMultiple
}

impl Claim {
    pub fn from_line(line: &String, regex: &Regex) -> Claim {
        let captures = regex.captures(line).unwrap();
        Claim {id: captures[1].parse().unwrap(),
                x: captures[2].parse().unwrap(),
                y: captures[3].parse().unwrap(),
                width: captures[4].parse().unwrap(), 
                height: captures[5].parse().unwrap()}
    }
}

fn initialize(lines: &Vec<String>) -> (Vec<Claim>, Array2<SquareState>) {
    let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();

    let claims = lines.iter().map(|l| Claim::from_line(l, &re)).collect::<Vec<Claim>>();
    let (max_x, max_y) = claims.iter()
        .map(|c| (c.x + c.width, c.y + c.height))
        .fold((0, 0), |(x1, y1), (x2, y2)| (max(x1, x2), max(y1, y2)));
    let square_array = Array2::from_elem((max_x, max_y), Untaken);

    return (claims, square_array);
}

fn calculate_square_states(claims: &Vec<Claim>, square_array: &mut Array2<SquareState>) {
    for claim in claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                square_array[(x, y)] = match square_array[(x, y)] {
                    Untaken => TakenOnce,
                    _ => TakenMultiple
                }
            }
        }
    }
}

pub fn star1(lines: &Vec<String>) -> String {
    let (claims, mut square_array) = initialize(lines);
    calculate_square_states(&claims, &mut square_array);

    square_array.iter()
        .filter(|state| **state == TakenMultiple)
        .count().to_string()
}

pub fn star2(lines: &Vec<String>) -> String {
    let (claims, mut square_array) = initialize(lines);
    calculate_square_states(&claims, &mut square_array);

    'claim_loop: for claim in claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if square_array[(x, y)] == TakenMultiple {
                    continue 'claim_loop;
                }
            }
        }
        return claim.id.to_string();
    }
    panic!("No solution found, invalid input!");
}