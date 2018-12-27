use regex::Regex;
use ndarray::prelude::*;

struct Star {
    x: i32,
    y: i32,
    speed_x: i32,
    speed_y: i32
}

const REGEX_STRING: &str = r"position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>";

fn parse_stars(lines: &Vec<String>) -> Vec<Star> {
    let regex = Regex::new(REGEX_STRING).unwrap();
    lines.iter().map(|line| {
        let captures = regex.captures(line).unwrap();
        Star {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
            speed_x: captures[3].parse().unwrap(),
            speed_y: captures[4].parse().unwrap(),
        }
    })
    .collect::<Vec<Star>>()
}

fn bounding_box(stars: &Vec<Star>) -> (i32, i32, i32, i32) {
    let min_x = stars.iter().map(|star| star.x).min().unwrap();
    let min_y = stars.iter().map(|star| star.y).min().unwrap();
    let max_x = stars.iter().map(|star| star.x).max().unwrap();
    let max_y = stars.iter().map(|star| star.y).max().unwrap();
    (min_x, min_y, max_x - min_x, max_y - min_y)
}

fn bounding_box_area(stars: &Vec<Star>) -> u64 {
    let (_, _, width, height) = bounding_box(stars);
    width as u64 * height as u64
}

fn update_stars(stars: &mut Vec<Star>) {
    for star in stars {
        star.x += star.speed_x;
        star.y += star.speed_y;
    }
}

fn revert_stars(stars: &mut Vec<Star>) {
    for star in stars {
        star.x -= star.speed_x;
        star.y -= star.speed_y;
    }
}

fn plot_stars(stars: &Vec<Star>) -> String {
    let (x, y, width, height) = bounding_box(&stars);
    let mut array = Array2::from_elem((width as usize + 1, height as usize + 1), '.');

    for star in stars {
        array[((star.x - x) as usize, (star.y - y) as usize)] = '#'
    }

    let mut plot = String::new();
    for row in array.gencolumns() {
        plot.push('\n');
        for &c in row {
            plot.push(c);
        }
    }   

    plot
}

fn run_simulation(lines: &Vec<String>) -> (Vec<Star>, i32) {
    let mut stars = parse_stars(lines);
    let mut area = bounding_box_area(&stars);
    update_stars(&mut stars);
    let mut new_area = bounding_box_area(&stars);
    let mut step = 1;
    while new_area <= area {
        area = new_area;
        update_stars(&mut stars);
        new_area = bounding_box_area(&stars);
        step += 1;
    }
    revert_stars(&mut stars);
    (stars, step - 1)
}

pub fn star1(lines: &Vec<String>) -> String {
    let (stars, _) = run_simulation(lines);

    plot_stars(&stars)
}

pub fn star2(lines: &Vec<String>) -> String {
    let (_, step_count) = run_simulation(lines);

    step_count.to_string()
}