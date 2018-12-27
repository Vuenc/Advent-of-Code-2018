use linked_list::LinkedList;

struct RingList<T> {
    vec: Vec<T>
}

impl<T> RingList<T> {
    fn new() -> RingList<T> {
        RingList {vec: Vec::new()}
    }

    fn vec_index(&self, index: i32) -> i32 {
        if self.vec.len() == 0 {
            0
        } else if index < 0 {
            self.vec.len() as i32 + (index % self.vec.len() as i32)
        } else {
            index % self.vec.len() as i32
        }
    }

    pub fn insert(&mut self, element: T, index: i32) -> i32 where T: std::fmt::Debug {
        let vec_index = self.vec_index(index);
        // println!("{:?}", self.vec);
        self.vec.insert(vec_index as usize, element);
        vec_index
    }

    pub fn remove(&mut self, index: i32) -> (T, i32) where T: std::fmt::Debug {
        let vec_index = self.vec_index(index);
        // println!("pre remove @ {}/{}: {:?}", vec_index, self.vec.len(), self.vec);
        (self.vec.remove(vec_index as usize), vec_index)
    }
}

#[allow(unused)]
fn get_winner_score_naive(player_count: usize, highest_marble: i32) -> i32 {
    let mut marbles = RingList::new();

    let mut current_marble_index = 0;
    let mut player_scores = vec![0; player_count];
    for marble in 0..(highest_marble + 1) {
        if marble == 0 || marble % 23 != 0 {
            current_marble_index = marbles.insert(marble, current_marble_index + 2);
        } else {
            let (removed_marble, new_current_marble_index) = marbles.remove(current_marble_index - 7);
            current_marble_index = new_current_marble_index;
            player_scores[marble as usize % player_count] += marble + removed_marble;
        }
    }

    let winner_score = player_scores.iter().max().unwrap();
    *winner_score
}

pub fn star1(lines: &Vec<String>) -> String {
    let player_count = lines[0].parse::<usize>().unwrap();
    let highest_marble = lines[1].parse::<i32>().unwrap();
    get_winner_score(player_count, highest_marble).to_string()
}

fn move_cursor<T>(cursor: &mut linked_list::Cursor<T>, positions: i32) {
    // LinkedList cycles by default, but has a None element between 
    // head and tail which must be skipped when encountered
    for _i in positions..0 {
        if let None = cursor.prev() {
            cursor.prev();
        }
    }
    for _i in 0..positions {
        if let None = cursor.next() {
            cursor.next();
        }
    }
}

fn get_winner_score(player_count: usize, highest_marble: i32) -> u64 {
    let mut marbles = LinkedList::new();
    let mut cursor = marbles.cursor();

    let mut player_scores = vec![0_u64; player_count];
    for marble in 0..(highest_marble + 1) {
        if marble == 0 || marble % 23 != 0 {
            move_cursor(&mut cursor, 2);
            cursor.insert(marble);
        } else {
            move_cursor(&mut cursor, -7);
            let removed_marble = cursor.remove().unwrap();
            player_scores[marble as usize % player_count] += (marble + removed_marble) as u64;
        }
    }

    let winner_score = player_scores.iter().max().unwrap();
    *winner_score
}

pub fn star2(lines: &Vec<String>) -> String {
    let player_count = lines[0].parse::<usize>().unwrap();
    let highest_marble = lines[1].parse::<i32>().unwrap() * 100;
    get_winner_score(player_count, highest_marble).to_string()
}