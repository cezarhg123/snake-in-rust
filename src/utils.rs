use crate::game::Direction;

pub fn read_file(filepath: &str) -> String {
    let file = std::fs::read_to_string(filepath).expect("failed to read file dipshit");
    file
}

pub fn has_movement(vector: &Vec<(i32, i32, Direction)>, loc: &(i32, i32)) -> bool {
    for x in vector {
        if x.0 == loc.0 && x.1 == loc.1 {
            return true;
        }
    };
    false
}

pub fn get_movement_index(vector: &Vec<(i32, i32, Direction)>, loc: &(i32, i32)) -> Option<usize> {
    for i in 0..vector.len() {
        if vector[i].0 == loc.0 && vector[i].1 == loc.1 {
            return Some(i);
        }
    };
    None
}