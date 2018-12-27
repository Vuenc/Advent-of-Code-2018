extern crate ndarray;

use crate::day2::ndarray::prelude::*;
use std::cmp::{min};

pub fn star1(lines: &Vec<String>) -> String {
    let character_lists = lines.into_iter().map(|l| l.chars().collect::<Vec<char>>());

    let mut two_count = 0;
    let mut three_count = 0;
    
    for mut characters in character_lists {
        characters.sort();
        let has_two_or_three = has_two_or_three_repetions(&characters);
        if has_two_or_three.0 {
            two_count += 1
        }
        if has_two_or_three.1 {
            three_count += 1
        }
    }

    return (two_count * three_count).to_string();
}

fn has_two_or_three_repetions(characters: &Vec<char>) -> (bool, bool) {
    let mut current_char = characters[0];
    let mut current_count = 1;
    let mut has_two_or_three = (false, false);
    for &c in &characters[1..] {
        if c == current_char {
            current_count += 1
        }
        else {
            if current_count == 2 {
                has_two_or_three.0 = true;
            }
            if current_count == 3 {
                has_two_or_three.1 = true;
            }
            if has_two_or_three.0 && has_two_or_three.1 {
                return has_two_or_three;
            }
            current_count = 1;
            current_char = c;
        }
    }
    has_two_or_three
}

pub fn star2(lines: &Vec<String>) -> String {
    let lines_reversed = lines.iter().map(|s| s.chars().rev().collect::<String>()).collect::<Vec<String>>();
    let mut lines_sorted = lines.iter().enumerate().collect::<Vec<(usize, &String)>>();
    let mut lines_reversed_sorted = lines_reversed.iter().enumerate().collect::<Vec<(usize, &String)>>();

    lines_sorted.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));
    lines_reversed_sorted.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));

    let mut matching_prefixes = Array2::from_elem((lines.len(), lines.len()), 0);
    let mut matching_postfixes = Array2::from_elem((lines.len(), lines.len()), 0);

    update_matching_pre_post_fixes(&lines_sorted, &mut matching_prefixes);
    update_matching_pre_post_fixes(&lines_reversed_sorted, &mut matching_postfixes);

    let max_element = matching_prefixes.indexed_iter().zip(matching_postfixes.indexed_iter())
        .map(|((index, prefix_length), (_, postfix_length))| (index, prefix_length + postfix_length))
        .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
        .unwrap();

    let solution_line1 = &lines[(max_element.0).0];
    let solution_line2 = &lines[(max_element.0).1];
    if solution_line1.len() != solution_line2.len() || max_element.1 != solution_line1.len() - 1 {
        panic!("Invalid output!");
    }
    let solution_prefix_length = common_prefix_length(&solution_line1, &solution_line2) as usize;
    return format!("{}{}", &solution_line1[..solution_prefix_length], &solution_line1[solution_prefix_length+1..]).to_string();
}

fn update_matching_pre_post_fixes(lines_sorted: &Vec<(usize, &String)>, matching_pre_post_fixes: &mut Array2<usize>) {
    for line_pair in lines_sorted.windows(2) {
        let ((index1, line1), (index2, line2)) = (line_pair[0], line_pair[1]);
        let common_length = common_prefix_length(line1, line2);
        
        for &(index_before, _) in lines_sorted.iter().take_while(|(i, _)| *i != index2) {

            let mut common_length_with_before =  common_length;
            if index_before != index1 {
                common_length_with_before = min(common_length, matching_pre_post_fixes[(index1, index_before)]);
            }

            matching_pre_post_fixes[(index2, index_before)] += common_length_with_before;
            matching_pre_post_fixes[(index_before, index2)] += common_length_with_before;
        }
    }
}

fn common_prefix_length(s1: &String, s2: &String) -> usize {
    let mut length = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            return length;
        }
        length += 1;
    }
    return length;
}

#[allow(unused)]
pub fn star2_naive(lines: &Vec<String>) -> String {
    let lines_reversed = lines.iter().map(|s| s.chars().rev().collect::<String>()).collect::<Vec<String>>();

    for (l1, l1r) in lines.iter().zip(&lines_reversed) {
        for (l2, l2r) in lines.iter().zip(&lines_reversed) {
            let prefix_length = common_prefix_length(&l1, &l2);
            let postfix_length = common_prefix_length(&l1r, &l2r);
            if prefix_length + postfix_length == l1.len() - 1 {
                return format!("{}{}", &l1[..prefix_length], &l1[prefix_length+1..]).to_string();
            }
        }
    }
    return "Nothing found!".to_string();
}