#![allow(unused)]

use std::cmp;
use regex::Regex;
use std::borrow::Cow;
use linked_list::LinkedList;
use std::iter::FromIterator;

fn character_pairs() -> Vec<(String, String)> {
    let characters = "abcdefghijklmnopqrstuvwxyz";
    characters.chars()
        .map(|c| (c.to_string(), c.to_uppercase().to_string()))
        .collect()
}

fn compile_reaction_regex() -> Regex {
    let mut regex_string = String::new();
    for (c1, c2) in character_pairs() {
        regex_string.push_str(&format!("{}{}|{}{}|", c1, c2, c2, c1));
    }
    Regex::new(&regex_string[..regex_string.len() - 1]).unwrap()
}

fn react(line: &String, reaction_regex: &Regex) -> String {
    let mut current_line = Cow::Borrowed(line);
    let mut old_length = current_line.len() + 1;
    while current_line.len() < old_length{
        old_length = current_line.len();
        current_line = Cow::Owned(reaction_regex.replace_all(&current_line, "").into());
    }
    current_line.into_owned()
}

pub fn star1_naive(line: &String) -> String {
    let reaction_regex = compile_reaction_regex();
    react(line, &reaction_regex).len().to_string()
}

fn compile_single_character_regexes() -> Vec<Regex> {
    character_pairs().iter()
        .map(|(c1, c2)| Regex::new(&format!("{}|{}", c1, c2)).unwrap())
        .collect()
}

pub fn star2_naive(line: &String) -> String {
    let mut min = line.len();
    let reaction_regex = compile_reaction_regex();
    let reacted_line = react(&line, &reaction_regex);
    for remove_character_regex in compile_single_character_regexes() {
        let stripped_line = remove_character_regex.replace_all(&reacted_line, "").into();
        let length_after_reaction = react(&stripped_line, &reaction_regex).len();
        min = cmp::min(min, length_after_reaction);
    }
    min.to_string()
}

fn react_quickly(line: &String) -> String {
    let mut char_list: LinkedList<_> = LinkedList::from_iter(line.chars());
    let mut changed = true;
    while changed {
        changed = false;
        let mut cursor = char_list.cursor();
        let mut current_char_option = cursor.next();
        while let Some(&mut current_char) = current_char_option {
            //let current_char = current_char_option.unwrap();//.clone();
            if let Some(&mut next_char) = cursor.peek_next() {
                if next_char == get_matching_char(current_char) {
                    cursor.seek_backward(1);
                    cursor.remove();
                    cursor.remove();
                    cursor.seek_backward(1);
                    changed = true
                }
            }
            current_char_option = cursor.next(); //TODO
        }
    }

    String::from_iter(char_list.into_iter())
}

fn get_matching_char(c: char) -> char {
    if c.is_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

pub fn star1(line: &String) -> String {
    react_quickly(line).len().to_string()
}

pub fn star2(line: &String) -> String {
    let mut min = line.len();
    let reacted_line = react_quickly(&line);
    for remove_character_regex in compile_single_character_regexes() {
        let stripped_line = remove_character_regex.replace_all(&reacted_line, "").into();
        let length_after_reaction = react_quickly(&stripped_line).len();
        min = cmp::min(min, length_after_reaction);
    }
    min.to_string()
}