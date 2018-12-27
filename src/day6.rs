use ndarray::prelude::*;
use regex::Regex;
use std::iter;

fn get_coordinates(lines: &Vec<String>) -> Vec<(i32, (usize, usize))> {
    let coordinates_regex = Regex::new(r"(\d*), (\d*)").unwrap();
    lines.iter()
        .enumerate()
        .map(|(id, line)| {
            let captures = coordinates_regex.captures(line).expect("Regex didn't match on line!");
            (id as i32, 
            (captures[1].parse().unwrap(), captures[2].parse().unwrap()))
        })
        .collect::<Vec<(i32, (usize, usize))>>()
}

fn normalize_coordinates(coordinates: &mut Vec<(i32, (usize, usize))>) -> (usize, usize) {
    let (&min_x, &min_y) = (coordinates.iter().map(|(_, (x, _))| x).min().unwrap(),
        coordinates.iter().map(|(_, (_, y))| y).min().unwrap());

    for (_, (x, y)) in coordinates.iter_mut() {
        *x = x.saturating_sub(min_x);
        *y = y.saturating_sub(min_y);
    }

    (*coordinates.iter().map(|(_, (x, _))| x).max().unwrap() + 1,
     *coordinates.iter().map(|(_, (_, y))| y).max().unwrap() + 1)    
}

fn distance((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> i32 {
    i32::abs(*x1 as i32 - *x2 as i32) + i32::abs(*y1 as i32 - *y2 as i32)
}

pub fn star1(lines: &Vec<String>) -> String {
    let mut coordinates = get_coordinates(lines);
    let (width, height) = normalize_coordinates(&mut coordinates);

    let mut closest_neighbor_array = Array2::from_elem((width, height), -1);

    for ((x, y), el) in closest_neighbor_array.indexed_iter_mut() {
        let mut closest = coordinates[0].0;
        let mut min_distance = distance(&coordinates[0].1, &(x, y));
        for coord in coordinates.iter().skip(1) {
            let dist = distance(&coord.1, &(x, y));
            if dist < min_distance {
                closest = coord.0;
                min_distance = dist;
            } else if dist == min_distance {
                closest = -1;
            }
        }
        *el = closest;
    }
    
    let mut finite_areas = vec![true; coordinates.len()];
    let border_coordinates = 
        (0..width).zip(iter::repeat(0)).chain(
        (0..width).zip(iter::repeat(height-1))).chain(
        iter::repeat(0).zip(0..height)).chain(
        iter::repeat(width-1).zip(0..height));
    
    for border_coord in border_coordinates {
        if closest_neighbor_array[border_coord] != -1 {
            finite_areas[closest_neighbor_array[border_coord] as usize] = false;
        }
    }

    let mut area_sizes =  vec![0; coordinates.len()];
    for &el in closest_neighbor_array.iter() {
        if el != -1 && finite_areas[el as usize] {
            area_sizes[el as usize] += 1;
        }
    }
    let hugest_area = area_sizes.iter().zip(finite_areas).filter(|(_, is_finite)| *is_finite).map(|(size, _)| size).max().unwrap();

    /* for &(id, (x, y)) in &coordinates {
        array[(x, y)] = -2;
    }
    let characters = "#.abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    for y in 0..height {
        for x in 0..width {

            print!("{}", characters[(array[(x,y)] + 2) as usize]);
        }
        println!();
    } */

    hugest_area.to_string()
}

pub fn star2(lines: &Vec<String>) -> String {
    let max_allowed_total_distance = 10000;

    let mut coordinates = get_coordinates(lines);
    let (width, height) = normalize_coordinates(&mut coordinates);

    let close_area_size = iproduct!(0..width, 0..height)
        .filter(|grid_coord| {
            coordinates.iter()
                .map(|(_, coord)| distance(grid_coord, coord))
                .sum::<i32>() < max_allowed_total_distance
        })
        .count();
    
    close_area_size.to_string()
}